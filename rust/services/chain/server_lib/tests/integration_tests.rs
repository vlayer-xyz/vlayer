use axum::http::StatusCode;
use chain_db::ChainDb;
use chain_server_lib::server;
use common::GuestElf;
use serde_json::json;
use server_utils::{body_to_json, post};

#[tokio::test]
async fn http_not_found() {
    let db = ChainDb::in_memory(GuestElf::default());
    let app = server(db);
    let empty_body = json!({});
    let response = post(app, "/non-existent", &empty_body).await;
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}

#[tokio::test]
async fn method_not_found() {
    let db = ChainDb::in_memory(GuestElf::default());
    let app = server(db);
    let req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "random_gibberish",
        "params": []
    });
    let response = post(app, "/", &req).await;

    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32601,
                "message": "Method `random_gibberish` not found",
            }
        }),
        body_to_json(response.into_body()).await
    );
}

#[tokio::test]
async fn method_missing() {
    let db = ChainDb::in_memory(GuestElf::default());
    let app = server(db);
    let req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "params": []
    });
    let response = post(app, "/", &req).await;

    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(
        json!({
            "jsonrpc": "2.0",
            "id": null,
            "error": {
                "code": -32600,
                "message": "missing field `method` at line 1 column 36",
            }
        }),
        body_to_json(response.into_body()).await
    );
}

mod chain_proof {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn success_dummy() {
        let db = ChainDb::in_memory(GuestElf::default());
        let app = server(db);
        let req = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "v_getChainProof",
            "params": {
                "chain_id": 1,
                "block_numbers": [1]
            }
        });
        let response = post(app, "/", &req).await;

        assert_eq!(StatusCode::OK, response.status());
        assert_eq!(
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": {
                    "proof": ["0x"],
                    "nodes": ["0x80"] // Null node RLP-encoded
                }
            }),
            body_to_json(response.into_body()).await
        );
    }

    #[tokio::test]
    async fn no_block_numbers_error() {
        let db = ChainDb::in_memory(GuestElf::default());
        let app = server(db);
        let req = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "v_getChainProof",
            "params": {
                "chain_id": 1,
                "block_numbers": []
            }
        });
        let response = post(app, "/", &req).await;

        assert_eq!(StatusCode::OK, response.status());
        assert_eq!(
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": -32602,
                    "message": "Invalid params: empty list of block numbers provided - nothing to prove",
                }
            }),
            body_to_json(response.into_body()).await
        );
    }

    #[tokio::test]
    async fn field_validation_error() {
        let db = ChainDb::in_memory(GuestElf::default());
        let app = server(db);

        let valid_number = 42;
        let invalid_number = "";
        let req = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "v_getChainProof",
            "params": {
                "chain_id": 1,
                "block_numbers": [valid_number, invalid_number]
            }
        });
        let response = post(app, "/", &req).await;

        assert_eq!(StatusCode::OK, response.status());
        assert_eq!(
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": -32602,
                    "message": "Invalid params",
                    "data": "invalid type: string \"\", expected u64 at line 1 column 36",
                }
            }),
            body_to_json(response.into_body()).await
        );
    }
}
