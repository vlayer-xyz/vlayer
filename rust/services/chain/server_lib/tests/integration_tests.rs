use axum::http::StatusCode;
use chain_db::ChainDb;
use chain_server_lib::server;
use common::GuestElf;
use serde_json::{Value, json};
use server_utils::{body_to_json, post};

fn test_app() -> axum::Router {
    let db = ChainDb::in_memory([GuestElf::default().id]);
    server(db)
}

#[tokio::test]
async fn http_not_found() {
    let app = test_app();
    let empty_body = json!({});
    let response = post(app, "/non-existent", &empty_body).await;
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}

#[tokio::test]
async fn method_not_found() {
    let app = test_app();
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
    let app = test_app();
    let req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "params": []
    });
    let response = post(app, "/", &req).await;

    assert_eq!(StatusCode::OK, response.status());
    let body = body_to_json(response.into_body()).await;
    assert_eq!(*body.get("id").unwrap(), Value::Null);
    let error = body.get("error").unwrap();
    assert_eq!(*error.get("code").unwrap(), json!(-32600));
    assert!(
        error
            .get("message")
            .unwrap()
            .as_str()
            .unwrap()
            .starts_with("missing field `method`")
    );
}

mod chain_proof {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn success_dummy() {
        let app = test_app();
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
        let app = test_app();
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
        let app = test_app();

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
        let body = body_to_json(response.into_body()).await;
        assert_eq!(*body.get("id").unwrap(), json!(1));
        let error = body.get("error").unwrap();
        assert_eq!(*error.get("code").unwrap(), json!(-32602));
        assert_eq!(*error.get("message").unwrap(), json!("Invalid params"));
        assert!(
            error
                .get("data")
                .unwrap()
                .as_str()
                .unwrap()
                .starts_with("invalid type: string \"\", expected u64")
        );
    }
}
