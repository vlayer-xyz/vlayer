use axum::http::StatusCode;
use serde_json::json;
use test_helpers::{call_guest_elf, chain_guest_elf, Context, API_VERSION, GAS_METER_TTL};

mod test_helpers;

use server_utils::{body_to_json, body_to_string};

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
        assert_eq!(
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": -32601,
                    "message": "Method `non_existent_json_rpc_method` not found",
                    "data": null
                }
            }),
            body_to_json(response.into_body()).await
        );
    }

    mod v_versions {
        use assert_json_diff::assert_json_include;
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
            assert_json_include!(
                expected: json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "call_guest_id": "0x0000000000000000000000000000000000000000000000000000000000000000",
                        "chain_guest_id": "0x0100000001000000010000000100000001000000010000000100000001000000",
                        "api_version": API_VERSION
                    }
                }),
                actual: body_to_json(response.into_body()).await,
            );
        }
    }

    mod v_call {
        use assert_json_diff::assert_json_include;
        use call_server::gas_meter::AllocateGas;
        use ethers::{
            abi::AbiEncode,
            types::{Uint8, U256},
        };
        use server_utils::{function_selector, RpcMethod, RpcServerMock};
        use web_proof::fixtures::{load_web_proof_v7_fixture, NOTARY_PUB_KEY_PEM_EXAMPLE};

        use super::*;
        use crate::test_helpers::mock::WebProof;

        const CHAIN_ID: u64 = 11155111;
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
            assert_eq!(
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "error": {
                        "code": -32602,
                        "message": "Invalid field: `to` Odd number of digits `I am not a valid address!`",
                        "data": null,
                    }
                }),
                body_to_json(response.into_body()).await
            );
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
            assert_json_include!(
                expected: json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
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
                    }
                }),
                actual: body_to_json(response.into_body()).await,
            );
        }

        #[tokio::test(flavor = "multi_thread")]
        async fn simple_with_gasmeter() {
            let mut ctx = Context::default().await;
            ctx.gas_meter_server = Some(
                RpcServerMock::start(
                    AllocateGas::METHOD_NAME,
                    false,
                    json!({
                        "gas_limit": GAS_LIMIT,
                        "hash": "0xf8d32367d8ec243e8e6fcac96dc769ed80287534d51c5d1e817173128f2b6218",
                        "time_to_live": GAS_METER_TTL
                    }),
                    json!({}),
                    1,
                )
                .await,
            );
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
        async fn success_web_proof() {
            let ctx = Context::default().await;
            let app = ctx.server(call_guest_elf(), chain_guest_elf());
            let contract = ctx.client.deploy_contract().await;

            let call_data = contract
                .web_proof(WebProof {
                    web_proof_json: serde_json::to_string(&json!(load_web_proof_v7_fixture(
                        "../../../web_proof/testdata/presentation.json",
                        NOTARY_PUB_KEY_PEM_EXAMPLE
                    )))
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
            assert_json_include!(
                expected: json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": {
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
                        }
                    }),
                actual: body_to_json(response.into_body()).await,
            );
        }
    }
}
