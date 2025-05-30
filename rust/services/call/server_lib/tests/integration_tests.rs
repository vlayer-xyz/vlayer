#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use axum::http::StatusCode;
use ethers::types::U256;
use serde_json::json;
use test_helpers::{API_VERSION, Context, call_guest_elf, chain_guest_elf, mock::GasMeterServer};

mod test_helpers;

use server_utils::{assert_jrpc_err, assert_jrpc_ok, body_to_json, body_to_string};
use test_helpers::{
    ETHEREUM_SEPOLIA_ID, GAS_LIMIT, GAS_METER_TTL, allocate_gas_body, rpc_body, v_call_body,
};

mod server_tests {
    use super::*;

    #[cfg(test)]
    #[ctor::ctor]
    fn before_all() {
        unsafe {
            std::env::set_var("RISC0_DEV_MODE", "1");
        }
    }

    #[tokio::test]
    async fn http_not_found() {
        let ctx = Context::default();
        let app = ctx.server(call_guest_elf(), chain_guest_elf());
        let response = app.post("/non_existent_http_path", &()).await;

        assert_eq!(StatusCode::NOT_FOUND, response.status());
        assert!(body_to_string(response.into_body()).await.is_empty());
    }

    #[tokio::test]
    async fn json_rpc_not_found() {
        let ctx = Context::default();
        let app = ctx.server(call_guest_elf(), chain_guest_elf());

        let req = rpc_body("non_existent_method", &json!([]));
        let response = app.post("/", &req).await;

        assert_eq!(StatusCode::OK, response.status());
        assert_jrpc_err(response, -32601, "Method `non_existent_method` not found").await;
    }

    mod v_versions {
        use common::GuestElf;
        use risc0_zkvm::get_version;

        use super::*;

        #[tokio::test]
        async fn success() {
            let call_elf = GuestElf::new([0; 8], &[]);
            let chain_elf = GuestElf::new([1; 8], &[]);
            let ctx = Context::default();
            let app = ctx.server(&call_elf, &chain_elf);

            let req = rpc_body("v_versions", &json!([]));
            let response = app.post("/", &req).await;

            assert_eq!(StatusCode::OK, response.status());
            assert_jrpc_ok(
                response,
                json!({
                    "call_guest_id": "0x0000000000000000000000000000000000000000000000000000000000000000",
                    "chain_guest_id": "0x0100000001000000010000000100000001000000010000000100000001000000",
                    "api_version": API_VERSION,
                    "risc0_version": get_version().unwrap().to_string(),
                }),
            ).await;
        }
    }

    mod v_call {
        use web_proof::fixtures::load_web_proof_fixture;

        use super::*;
        use crate::test_helpers::mock::WebProof;

        #[tokio::test]
        async fn field_validation_error() {
            let ctx = Context::default();
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.deploy_contract().await;
            let call_data = contract
                .sum(U256::from(1), U256::from(2))
                .calldata()
                .unwrap();

            let req = json!({
                "method": "v_call",
                "params": [
                    {
                        "to": "I am not a valid address!",
                        "data": call_data,
                        "gas_limit": GAS_LIMIT,
                    },
                    {
                        "chain_id": ETHEREUM_SEPOLIA_ID,
                    }
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = app.post("/", &req).await;

            assert_eq!(StatusCode::OK, response.status());
            assert_jrpc_err(
                response,
                -32602,
                "Invalid field: `to` Odd number of digits `I am not a valid address!`",
            )
            .await;
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn simple_contract_call_success() {
            const EXPECTED_HASH: &str =
                "0x0172834e56827951e1772acaf191c488ba427cb3218d251987a05406ec93f2b2";

            let ctx = Context::default();
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.deploy_contract().await;
            let call_data = contract
                .sum(U256::from(1), U256::from(2))
                .calldata()
                .unwrap();

            let req = v_call_body(contract.address(), &call_data);
            let response = app.post("/", &req).await;

            assert_eq!(StatusCode::OK, response.status());
            assert_jrpc_ok(response, EXPECTED_HASH).await;
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn web_proof_success() {
            const EXPECTED_HASH: &str =
                "0xabbb6b3fc90283cf623ba085d24288708c3f2d1ee231f2ce17a60574fb7ee664";

            let ctx = Context::default();
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.deploy_contract().await;

            // We use serde_json "preserve_order" feature to ensure that the expected hash is deterministic
            let web_proof_json = serde_json::to_string(&json!(load_web_proof_fixture())).unwrap();
            let web_proof = WebProof { web_proof_json };
            let call_data = contract.web_proof(web_proof).calldata().unwrap();

            let req = v_call_body(contract.address(), &call_data);

            let response = app.post("/", &req).await;

            assert_eq!(StatusCode::OK, response.status());
            assert_jrpc_ok(response, EXPECTED_HASH).await;
        }
    }

    #[allow(non_snake_case)]
    mod v_getProofReceipt {
        use alloy_primitives::B256;
        use assert_json_diff::assert_json_include;
        use call_server_lib::{v_call::CallHash, v_get_proof_receipt::State};
        use ethers::{
            abi::AbiEncode,
            types::{Bytes, H160, Uint8},
        };
        use serde_json::Value;
        use server_utils::function_selector;
        use tower::{ServiceBuilder, ServiceExt};
        use web_proof::fixtures::load_web_proof_fixture;

        use super::*;
        use crate::test_helpers::mock::{Contract, Server, WebProof};

        const RETRY_SLEEP_DURATION: tokio::time::Duration = tokio::time::Duration::from_millis(100);
        const MAX_POLLING_TIME: std::time::Duration = std::time::Duration::from_secs(60);

        type Req = Value;
        type Resp = (State, bool, Value);

        #[derive(Clone)]
        struct RetryRequest;

        impl tower::retry::Policy<Req, Resp, String> for RetryRequest {
            type Future = tokio::time::Sleep;

            fn retry(
                &mut self,
                _req: &mut Req,
                result: &mut Result<Resp, String>,
            ) -> Option<Self::Future> {
                result
                    .as_ref()
                    .ok()
                    .and_then(|(state, success, _)| match state {
                        State::Preflight | State::Proving => {
                            if !success {
                                None
                            } else {
                                Some(tokio::time::sleep(RETRY_SLEEP_DURATION))
                            }
                        }
                        State::Done => None,
                        _ => Some(tokio::time::sleep(RETRY_SLEEP_DURATION)),
                    })
            }

            fn clone_request(&mut self, req: &Req) -> Option<Req> {
                Some(req.clone())
            }
        }

        async fn get_hash(
            app: &Server,
            contract: &Contract,
            call_data: &Bytes,
        ) -> call_server_lib::v_call::CallHash {
            let request = v_call_body(contract.address(), call_data);
            let response = app.post("/", &request).await;
            assert_eq!(StatusCode::OK, response.status());
            let as_json = body_to_json(response.into_body()).await;
            serde_json::from_value(as_json["result"].clone())
                .expect("valid returned hash value of the call params")
        }

        fn v_get_proof_receipt_body(hash: CallHash) -> Value {
            json!({
                    "method": "v_getProofReceipt",
                    "params": { "hash": hash },
                    "id": 1,
                    "jsonrpc": "2.0",
            })
        }

        async fn v_get_proof_receipt_result(app: &Server, request: Req) -> (State, bool, Value) {
            let response = app.post("/", &request).await;
            assert_eq!(StatusCode::OK, response.status());
            let result = assert_jrpc_ok(response, json!({})).await;
            let state: State = serde_json::from_value(result["result"]["state"].clone())
                .expect("state should be a valid enum variant");
            let status: u8 = serde_json::from_value(result["result"]["status"].clone())
                .expect("status should be a valid u8 value");
            (state, status == 1, result["result"].clone())
        }

        async fn get_proof_result(app: &Server, hash: CallHash) -> Value {
            let svc = ServiceBuilder::new()
                .layer(tower::timeout::TimeoutLayer::new(MAX_POLLING_TIME))
                .layer(tower::retry::RetryLayer::new(RetryRequest))
                .service_fn(|request| async move {
                    Ok(v_get_proof_receipt_result(app, request).await) as Result<(_, _, _), String>
                });
            let (_, _, result) = svc.oneshot(v_get_proof_receipt_body(hash)).await.unwrap();
            result
        }

        fn assert_proof_result(
            result: &Value,
            evm_call_result: impl Into<Value>,
            call_data: &Bytes,
            contract_address: H160,
        ) {
            assert_json_include!(
                actual: result,
                expected: json!({
                    "state": "done",
                    "status": 1,
                    "data": {
                        "evm_call_result": evm_call_result.into(),
                        "proof": {
                            "length": 192,
                            "seal": {
                                "verifierSelector": "0xdeafbeef",
                                "mode": 1,
                            },
                            "callAssumptions": {
                                "functionSelector": function_selector(call_data),
                                "proverContractAddress": contract_address,
                            }
                        }
                    },
                    "metrics": {},
                }),
            );
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn nonexistent_hash_failure() {
            let ctx = Context::default();
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let fake_hash = CallHash::from(B256::repeat_byte(0xaa));
            let response = app.post("/", v_get_proof_receipt_body(fake_hash)).await;
            assert_jrpc_err(response, -32600, &format!("Hash not found: {fake_hash}")).await;
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn invalid_contract_preflight_error() {
            let ctx = Context::default();
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.deploy_contract().await;
            let call_data = Bytes::from_static(b"Hello world");

            let hash = get_hash(&app, &contract, &call_data).await;
            let result = get_proof_result(&app, hash).await;

            assert_json_include!(actual: result, expected: json!({
                "state": "preflight",
                "error": "Preflight: Transaction reverted: <empty>. This can happen for multiple reasons:
    - Call to contract with no code. Please make sure the prover contract address is correct.
    - Calling revert() or require() without a revert reason.
    - Assertions without a revert reason: assert(false).
    - Out-of-Gas exceptions.
    - Invalid opcodes (e.g. division by zero).
    - Some precompile errors.
    ",
                "status": 0,
                "metrics": {},
            }));
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn two_subsequent_calls_when_ready_success() {
            let ctx = Context::default();
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.deploy_contract().await;
            let call_data = contract
                .sum(U256::from(1), U256::from(2))
                .calldata()
                .unwrap();

            let hash = get_hash(&app, &contract, &call_data).await;
            let result = get_proof_result(&app, hash).await;

            assert_proof_result(
                &result,
                U256::from(3).encode_hex(),
                &call_data,
                contract.address(),
            );

            let (state, status, result) =
                v_get_proof_receipt_result(&app, v_get_proof_receipt_body(hash)).await;

            assert_eq!(state, State::Done);
            assert!(status);
            assert_proof_result(
                &result,
                U256::from(3).encode_hex(),
                &call_data,
                contract.address(),
            );
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn simple_contract_call_success() {
            let ctx = Context::default();
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.deploy_contract().await;
            let call_data = contract
                .sum(U256::from(1), U256::from(2))
                .calldata()
                .unwrap();

            let hash = get_hash(&app, &contract, &call_data).await;
            let result = get_proof_result(&app, hash).await;
            assert_proof_result(
                &result,
                U256::from(3).encode_hex(),
                &call_data,
                contract.address(),
            );
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn web_proof_success() {
            let ctx = Context::default();
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.deploy_contract().await;
            let call_data = contract
                .web_proof(WebProof {
                    web_proof_json: serde_json::to_string(&json!(load_web_proof_fixture()))
                        .unwrap(),
                })
                .calldata()
                .unwrap();

            let hash = get_hash(&app, &contract, &call_data).await;
            let result = get_proof_result(&app, hash).await;
            assert_proof_result(
                &result,
                Uint8::from(1).encode_hex(),
                &call_data,
                contract.address(),
            );
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn simple_with_gas_meter() {
            const EXPECTED_HASH: &str =
                "0x0172834e56827951e1772acaf191c488ba427cb3218d251987a05406ec93f2b2";
            const EXPECTED_GAS_USED: u64 = 21_724;

            let mut gas_meter_server = GasMeterServer::start(GAS_METER_TTL, None).await;
            gas_meter_server
                .mock_method("v_allocateGas")
                .with_params(allocate_gas_body(EXPECTED_HASH), false)
                .with_result(json!({}))
                .add()
                .await;
            gas_meter_server
                .mock_method("v_refundUnusedGas")
                .with_params(
                    json!({
                        "hash": EXPECTED_HASH,
                        "computation_stage": "preflight",
                        "gas_used": EXPECTED_GAS_USED,
                    }),
                    false,
                )
                .with_result(json!({}))
                .add()
                .await;
            gas_meter_server
                .mock_method("v_refundUnusedGas")
                .with_params(
                    json!({
                        "hash": EXPECTED_HASH,
                        "computation_stage": "proving",
                        "gas_used": EXPECTED_GAS_USED,
                    }),
                    false,
                )
                .with_result(json!({}))
                .add()
                .await;
            gas_meter_server
                .mock_method("v_sendMetadata")
                .with_params(
                    json!({
                        "hash": EXPECTED_HASH,
                        "metadata": [{"start_chain": ETHEREUM_SEPOLIA_ID}]
                    }),
                    false,
                )
                .with_result(json!({}))
                .add()
                .await;

            let ctx = Context::default().with_gas_meter_server(gas_meter_server);
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.deploy_contract().await;
            let call_data = contract
                .sum(U256::from(1), U256::from(2))
                .calldata()
                .unwrap();

            let hash = get_hash(&app, &contract, &call_data).await;
            get_proof_result(&app, hash).await;

            ctx.assert_gas_meter();
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn ensure_we_allocate_gas_only_once() {
            const EXPECTED_HASH: &str =
                "0x0172834e56827951e1772acaf191c488ba427cb3218d251987a05406ec93f2b2";

            let mut gas_meter_server = GasMeterServer::start(GAS_METER_TTL, None).await;
            gas_meter_server
                .mock_method("v_allocateGas")
                .with_params(allocate_gas_body(EXPECTED_HASH), false)
                .with_result(json!({}))
                .with_expected_calls(1)
                .add()
                .await;

            let ctx = Context::default().with_gas_meter_server(gas_meter_server);
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.deploy_contract().await;
            let call_data = contract
                .sum(U256::from(1), U256::from(2))
                .calldata()
                .unwrap();

            let _hash = get_hash(&app, &contract, &call_data).await;
            let _hash = get_hash(&app, &contract, &call_data).await;

            ctx.assert_gas_meter();
        }
    }

    mod jwt {
        use assert_json_diff::assert_json_eq;
        use server_utils::jwt::{
            ClaimsBuilder, EncodingKey, Environment, Header, encode, get_current_timestamp,
            test_helpers::{
                JWT_SECRET, TokenArgs, default_config as default_jwt_config, token as test_token,
            },
        };
        use test_helpers::mock::Server;

        use super::*;

        fn token(invalid_after: i64, subject: &str) -> String {
            test_token(&TokenArgs {
                secret: JWT_SECRET,
                host: Some("api.vlayer.xyz"),
                port: None,
                invalid_after,
                subject,
                environment: None,
            })
        }

        fn default_app() -> Server {
            Context::default()
                .with_jwt_auth(default_jwt_config())
                .server(call_guest_elf(), chain_guest_elf())
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn accepts_requests_with_valid_token() {
            let app = default_app();
            let req = rpc_body("dummy", &json!([]));
            let resp = app
                .post_with_bearer_auth("/", &req, &token(60, "1234"))
                .await;

            assert_eq!(StatusCode::OK, resp.status());
            assert_jrpc_err(resp, -32601, "Method `dummy` not found").await;
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn rejects_requests_with_missing_token() {
            let app = default_app();
            let req = rpc_body("dummy", &json!([]));
            let resp = app.post("/", &req).await;

            assert_eq!(StatusCode::UNAUTHORIZED, resp.status());
            assert_json_eq!(
                body_to_json(resp.into_body()).await,
                json!({ "error": "Missing JWT token" })
            );
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn rejects_requests_with_expired_token() {
            let app = default_app();
            let req = rpc_body("dummy", &json!([]));
            let resp = app
                .post_with_bearer_auth("/", &req, &token(-120, "1234"))
                .await;

            assert_eq!(StatusCode::UNAUTHORIZED, resp.status());
            assert_json_eq!(
                body_to_json(resp.into_body()).await,
                json!({ "error": "ExpiredSignature" })
            );
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn rejects_requests_with_tampered_with_token() {
            let key = EncodingKey::from_secret(b"beefdead");
            let ts = get_current_timestamp() + 1000;
            let claims = ClaimsBuilder::default()
                .exp(ts)
                .sub("1234".to_string())
                .build()
                .unwrap();
            let token = encode(&Header::default(), &claims, &key).unwrap();

            let app = default_app();
            let req = rpc_body("dummy", &json!([]));
            let resp = app.post_with_bearer_auth("/", &req, &token).await;

            assert_eq!(StatusCode::UNAUTHORIZED, resp.status());
            assert_json_eq!(
                body_to_json(resp.into_body()).await,
                json!({ "error": "InvalidSignature" })
            );
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn rejects_requests_with_old_token() {
            const OLD_TOKEN: &str = "sk_1234567890";

            let app = default_app();
            let req = rpc_body("dummy", &json!([]));
            let resp = app.post_with_bearer_auth("/", &req, OLD_TOKEN).await;

            assert_eq!(StatusCode::UNAUTHORIZED, resp.status());
            assert_json_eq!(
                body_to_json(resp.into_body()).await,
                json!({ "error": "Invalid JWT token" })
            );
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn reject_requests_with_mismatched_environment() {
            let key = EncodingKey::from_secret(JWT_SECRET.as_bytes());
            let ts = get_current_timestamp() + 1000;
            let claims = ClaimsBuilder::default()
                .exp(ts)
                .sub("1234".to_string())
                .environment(Environment::Production)
                .build()
                .unwrap();
            let token = encode(&Header::default(), &claims, &key).unwrap();

            let app = default_app();
            let req = rpc_body("dummy", &json!([]));
            let resp = app.post_with_bearer_auth("/", &req, &token).await;

            assert_eq!(StatusCode::BAD_REQUEST, resp.status());
            assert_json_eq!(
                body_to_json(resp.into_body()).await,
                json!({ "error": "Invalid environment in JWT: production, prover server proof mode: fake" }),
            );
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn authenticates_with_gas_meter() {
            const API_KEY_HEADER_NAME: &str = "x-prover-api-key";
            const API_KEY: &str = "secret-deadbeef";
            const EXPECTED_HASH: &str =
                "0x0172834e56827951e1772acaf191c488ba427cb3218d251987a05406ec93f2b2";
            const SUBJECT: &str = "1234";
            const SLEEP_DURATION: tokio::time::Duration = tokio::time::Duration::from_millis(100);

            let mut gas_meter_server =
                GasMeterServer::start(GAS_METER_TTL, Some(API_KEY.into())).await;
            gas_meter_server
                .mock_method("v_allocateGas")
                .with_bearer_auth(SUBJECT)
                .with_params(allocate_gas_body(EXPECTED_HASH), false)
                .with_result(json!({}))
                .with_expected_header(API_KEY_HEADER_NAME, API_KEY)
                .add()
                .await;

            let jwt_config = default_jwt_config();
            let ctx = Context::default()
                .with_jwt_auth(jwt_config)
                .with_gas_meter_server(gas_meter_server);
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.deploy_contract().await;
            let call_data = contract
                .sum(U256::from(1), U256::from(2))
                .calldata()
                .unwrap();

            let req = v_call_body(contract.address(), &call_data);
            let response = app
                .post_with_bearer_auth("/", &req, &token(60, SUBJECT))
                .await;

            assert_eq!(StatusCode::OK, response.status());
            assert_jrpc_ok(response, EXPECTED_HASH).await;

            tokio::time::sleep(SLEEP_DURATION).await;
            ctx.assert_gas_meter();
        }
    }
}
