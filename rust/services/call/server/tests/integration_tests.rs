use axum::http::StatusCode;
use serde_json::json;
use test_helpers::{call_guest_elf, chain_guest_elf, Context, API_VERSION, GAS_METER_TTL};

mod test_helpers;

use server_utils::{
    assert_jrpc_err, assert_jrpc_ok, body_to_json, body_to_string, function_selector,
};

mod server_tests {
    use super::*;

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

        let req = json!({
            "method": "non_existent_json_rpc_method",
            "params": [],
            "id": 1,
            "jsonrpc": "2.0",
        });
        let response = app.post("/", &req).await;

        assert_eq!(StatusCode::OK, response.status());
        assert_jrpc_err(response, -32601, "Method `non_existent_json_rpc_method` not found").await;
    }

    mod v_versions {
        use common::GuestElf;

        use super::*;

        #[tokio::test]
        async fn success() {
            let call_elf = GuestElf::new([0; 8], &[]);
            let chain_elf = GuestElf::new([1; 8], &[]);
            let ctx = Context::default().await;
            let app = ctx.server(call_elf, chain_elf);

            let req = json!({
                "method": "v_versions",
                "params": [],
                "id": 1,
                "jsonrpc": "2.0",
            });
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
        use server_utils::rpc::mock::Server as RpcServerMock;
        use web_proof::fixtures::load_web_proof_fixture;

        use super::*;
        use crate::test_helpers::mock::WebProof;

        const CHAIN_ID: u64 = 11_155_111;
        const GAS_LIMIT: u64 = 1_000_000;

        #[tokio::test]
        async fn field_validation_error() {
            let ctx = Context::default().await;
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.client.deploy_contract().await;
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
                    }, {
                        "gas_limit": GAS_LIMIT,
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
                "0x126257b312be17f869dacc198adc28424148f5408751f52c50050a01eeef8ebf";

            let ctx = Context::default().await;
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.client.deploy_contract().await;
            let call_data = contract
                .sum(U256::from(1), U256::from(2))
                .calldata()
                .unwrap();

            let req = json!({
            "method": "v_call",
            "params": [
                {
                    "to": contract.address(),
                    "data": call_data,
                },
                {
                    "chain_id": CHAIN_ID ,
                    "gas_limit": GAS_LIMIT,
                }
                ],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = app.post("/", &req).await;

            assert_eq!(StatusCode::OK, response.status());
            assert_jrpc_ok(response, EXPECTED_HASH).await;
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn simple_with_gasmeter() {
            const EXPECTED_HASH: &str =
                "0x126257b312be17f869dacc198adc28424148f5408751f52c50050a01eeef8ebf";
            const EXPECTED_GAS_USED: u64 = 21_728;

            let mut gas_meter_server = RpcServerMock::start().await;
            gas_meter_server
                .mock_method("v_allocateGas")
                .with_params(
                    json!({
                        "gas_limit": GAS_LIMIT,
                        "hash": EXPECTED_HASH,
                        "time_to_live": GAS_METER_TTL
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

            let mut ctx = Context::default().await;
            ctx.gas_meter_server = Some(gas_meter_server);
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.client.deploy_contract().await;

            let call_data = contract
                .sum(U256::from(1), U256::from(2))
                .calldata()
                .unwrap();

            let req = json!({
                "method": "v_call",
                "params": [
                    {
                        "to": contract.address(),
                        "data": call_data,
                    },
                    {
                        "chain_id": CHAIN_ID ,
                        "gas_limit": GAS_LIMIT,
                    }
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = app.post("/", &req).await;
            assert_eq!(StatusCode::OK, response.status());
            ctx.gas_meter_server.unwrap().assert();
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn web_proof_success() {
            const EXPECTED_HASH: &str =
                "0x6a7ef4881bdb500aa57be618a4594af28f7d812ac11088b29912442e986a8853";

            let ctx = Context::default().await;
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.client.deploy_contract().await;

            let call_data = contract
                .web_proof(WebProof {
                    web_proof_json: serde_json::to_string(&json!(load_web_proof_fixture()))
                        .unwrap(),
                })
                .calldata()
                .unwrap();

            let req = json!({
                "method": "v_call",
                "params": [
                    {
                        "to": contract.address(),
                        "data": call_data,
                    },
                    {
                        "chain_id": CHAIN_ID,
                        "gas_limit": GAS_LIMIT,
                    }
                    ],
                "id": 1,
                "jsonrpc": "2.0",
            });

            let response = app.post("/", &req).await;

            assert_eq!(StatusCode::OK, response.status());
            assert_jrpc_ok(response, EXPECTED_HASH).await;
        }
    }

    #[allow(non_snake_case)]
    mod v_getProofReceipt {
        use ethers::{
            abi::AbiEncode,
            types::{Bytes, Uint8, U256},
        };
        use web_proof::fixtures::load_web_proof_fixture;

        use super::*;
        use crate::test_helpers::mock::{Contract, Server, WebProof};

        const CHAIN_ID: u64 = 11_155_111;
        const GAS_LIMIT: u64 = 1_000_000;

        async fn get_hash(
            app: &Server,
            contract: &Contract,
            call_data: &Bytes,
        ) -> call_server::v_call::CallHash {
            let request = json!({
                "method": "v_call",
                "params": [
                    {
                        "to": contract.address(),
                        "data": call_data,
                    },
                    {
                        "chain_id": CHAIN_ID ,
                        "gas_limit": GAS_LIMIT,
                    }
                ],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = app.post("/", &request).await;
            assert_eq!(StatusCode::OK, response.status());
            let as_json = body_to_json(response.into_body()).await;
            serde_json::from_value(as_json["result"].clone())
                .expect("valid returned hash value of the call params")
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn simple_contract_call_success() {
            let ctx = Context::default().await;
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.client.deploy_contract().await;
            let call_data = contract
                .sum(U256::from(1), U256::from(2))
                .calldata()
                .unwrap();

            let hash = get_hash(&app, &contract, &call_data).await;

            let request = json!({
                "method": "v_getProofReceipt",
                "params": { "hash": hash },
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = app.post("/", &request).await;
            assert_eq!(StatusCode::OK, response.status());
            assert_jrpc_ok(
                response,
                json!({
                    "evm_call_result": U256::from(3).encode_hex(),
                    "proof": {
                        "length": 160,
                        "seal": {
                            "verifierSelector": "0xdeafbeef",
                            "mode": 1,
                        },
                        "callAssumptions": {
                            "functionSelector": function_selector(&call_data),
                            "proverContractAddress": contract.address(),
                        }
                    },
                }),
            )
            .await;
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn web_proof_success() {
            let ctx = Context::default().await;
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.client.deploy_contract().await;
            let call_data = contract
                .web_proof(WebProof {
                    web_proof_json: serde_json::to_string(&json!(load_web_proof_fixture()))
                        .unwrap(),
                })
                .calldata()
                .unwrap();

            let hash = get_hash(&app, &contract, &call_data).await;

            let request = json!({
                "method": "v_getProofReceipt",
                "params": { "hash": hash },
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = app.post("/", &request).await;
            assert_eq!(StatusCode::OK, response.status());
            assert_jrpc_ok(
                response,
                json!({
                    "evm_call_result": Uint8::from(1).encode_hex(),
                    "proof": {
                        "length": 160,
                        "seal": {
                            "verifierSelector": "0xdeafbeef",
                            "mode": 1,
                        },
                        "callAssumptions": {
                            "functionSelector": function_selector(&call_data),
                            "proverContractAddress": contract.address(),
                        }
                    },
                }),
            )
            .await;
        }
    }
}
