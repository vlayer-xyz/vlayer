use axum::http::StatusCode;
use serde_json::json;
use test_helpers::{call_guest_elf, chain_guest_elf, mock::GasMeterServer, Context, API_VERSION};

mod test_helpers;

use server_utils::{assert_jrpc_err, assert_jrpc_ok, body_to_json, body_to_string};
use test_helpers::{
    allocate_gas_body, rpc_body, v_call_body, ETHEREUM_SEPOLIA_ID, GAS_LIMIT, GAS_METER_TTL,
};

mod server_tests {
    use super::*;

    #[cfg(test)]
    #[ctor::ctor]
    fn before_all() {
        std::env::set_var("RISC0_DEV_MODE", "1");
    }

    #[tokio::test]
    async fn http_not_found() {
        let ctx = Context::default().await;
        let app = ctx.server(call_guest_elf(), chain_guest_elf());
        let response = app.post("/non_existent_http_path", &()).await;

        assert_eq!(StatusCode::NOT_FOUND, response.status());
        assert!(body_to_string(response.into_body()).await.is_empty());
    }

    #[tokio::test]
    async fn json_rpc_not_found() {
        let ctx = Context::default().await;
        let app = ctx.server(call_guest_elf(), chain_guest_elf());

        let req = rpc_body("non_existent_method", &json!([]));
        let response = app.post("/", &req).await;

        assert_eq!(StatusCode::OK, response.status());
        assert_jrpc_err(response, -32601, "Method `non_existent_method` not found").await;
    }

    mod v_versions {
        use common::GuestElf;

        use super::*;

        #[tokio::test]
        async fn success() {
            let call_elf = GuestElf::new([0; 8], &[]);
            let chain_elf = GuestElf::new([1; 8], &[]);
            let ctx = Context::default().await;
            let app = ctx.server(call_elf, &chain_elf);

            let req = rpc_body("v_versions", &json!([]));
            let response = app.post("/", &req).await;

            assert_eq!(StatusCode::OK, response.status());
            assert_jrpc_ok(
                response,
                json!({
                    "call_guest_id": "0x0000000000000000000000000000000000000000000000000000000000000000",
                    "chain_guest_id": "0x0100000001000000010000000100000001000000010000000100000001000000",
                    "api_version": API_VERSION
                }),
            ).await;
        }
    }

    mod v_call {
        use ethers::types::U256;
        use web_proof::fixtures::load_web_proof_fixture;

        use super::*;
        use crate::test_helpers::mock::WebProof;

        #[tokio::test]
        async fn field_validation_error() {
            let mut ctx = Context::default().await;
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

            let mut ctx = Context::default().await;
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
                "0x1a1fac6c674fd5a09b9a1c3df14eb6ea34786f0707eee014e1f9200dec9f380e";

            let mut ctx = Context::default().await;
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.deploy_contract().await;

            let call_data = contract
                .web_proof(WebProof {
                    web_proof_json: serde_json::to_string(&json!(load_web_proof_fixture()))
                        .unwrap(),
                })
                .calldata()
                .unwrap();

            let req = v_call_body(contract.address(), &call_data);

            let response = app.post("/", &req).await;

            assert_eq!(StatusCode::OK, response.status());
            assert_jrpc_ok(response, EXPECTED_HASH).await;
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn gasmeter_with_api_key() {
            const EXPECTED_HASH: &str =
                "0x0172834e56827951e1772acaf191c488ba427cb3218d251987a05406ec93f2b2";
            const API_KEY_HEADER_NAME: &str = "x-prover-api-key";
            const API_KEY: &str = "secret-deadbeef";

            let mut gas_meter_server =
                GasMeterServer::start(GAS_METER_TTL, Some(API_KEY.into())).await;
            gas_meter_server
                .mock_method("v_allocateGas")
                .with_params(allocate_gas_body(EXPECTED_HASH), false)
                .with_result(json!({}))
                .with_expected_header(API_KEY_HEADER_NAME, API_KEY)
                .add()
                .await;

            let mut ctx = Context::default()
                .await
                .with_gas_meter_server(gas_meter_server);
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

            ctx.assert_gas_meter();
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn gasmeter_with_user_token() {
            const EXPECTED_HASH: &str =
                "0x0172834e56827951e1772acaf191c488ba427cb3218d251987a05406ec93f2b2";
            const USER_TOKEN: &str = "sk_1234567890";

            let mut gas_meter_server = GasMeterServer::start(GAS_METER_TTL, None).await;
            gas_meter_server
                .mock_method("v_allocateGas")
                .with_bearer_auth(USER_TOKEN)
                .with_params(allocate_gas_body(EXPECTED_HASH), false)
                .with_result(json!({}))
                .add()
                .await;

            let mut ctx = Context::default()
                .await
                .with_gas_meter_server(gas_meter_server);
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.deploy_contract().await;
            let call_data = contract
                .sum(U256::from(1), U256::from(2))
                .calldata()
                .unwrap();

            let req = v_call_body(contract.address(), &call_data);
            let response = app.post_with_bearer_auth("/", &req, USER_TOKEN).await;

            assert_eq!(StatusCode::OK, response.status());
            assert_jrpc_ok(response, EXPECTED_HASH).await;

            ctx.assert_gas_meter();
        }
    }

    #[allow(non_snake_case)]
    mod v_getProofReceipt {
        use alloy_primitives::B256;
        use assert_json_diff::assert_json_include;
        use call_server_lib::{v_call::CallHash, v_get_proof_receipt::State};
        use ethers::{
            abi::AbiEncode,
            types::{Bytes, Uint8, H160, U256},
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
                        State::ChainProof | State::Preflight | State::Proving => {
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
                            "length": 160,
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
            let ctx = Context::default().await;
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let fake_hash = CallHash::from(B256::repeat_byte(0xaa));
            let response = app.post("/", v_get_proof_receipt_body(fake_hash)).await;
            assert_jrpc_err(response, -32600, &format!("Hash not found: {fake_hash}")).await;
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn invalid_contract_preflight_error() {
            let mut ctx = Context::default().await;
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.deploy_contract().await;
            let call_data = Bytes::from_static(b"Hello world");

            let hash = get_hash(&app, &contract, &call_data).await;
            let result = get_proof_result(&app, hash).await;

            assert_json_include!(actual: result, expected: json!({
                "state": "preflight",
                "error": "Preflight error: TravelCallExecutor error: EVM transact error: ",
                "status": 0,
                "metrics": {},
            }));
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn two_subsequent_calls_when_ready_success() {
            let mut ctx = Context::default().await;
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
            let mut ctx = Context::default().await;
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
            let mut ctx = Context::default().await;
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
        async fn simple_with_gasmeter() {
            const EXPECTED_HASH: &str =
                "0x0172834e56827951e1772acaf191c488ba427cb3218d251987a05406ec93f2b2";
            const EXPECTED_GAS_USED: u64 = 21_728;

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

            let mut ctx = Context::default()
                .await
                .with_gas_meter_server(gas_meter_server);
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
    }
}
