use std::str::FromStr;

use reqwest::Url;
use serde::Deserialize;
use serde_json::json;
use strum::EnumString;
use vercel_runtime::{Body, Error, Request, Response, StatusCode, run};
use web_prover::{
    NotarizeParams, NotarizeParamsBuilder, NotaryConfig, NotaryConfigBuilder, generate_web_proof,
};

#[derive(Debug, PartialEq, Eq, EnumString)]
enum Scheme {
    #[strum(serialize = "http")]
    Http,
    #[strum(serialize = "https")]
    Https,
}

const DEFAULT_NOTARY_URL: &str = "https://test-notary.vlayer.xyz/v0.1.0-alpha.11";
const DEFAULT_MAX_SENT_DATA: usize = 1 << 12;
const DEFAULT_MAX_RECV_DATA: usize = 1 << 14;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum Method {
    Get,
    Post,
}

impl From<Method> for web_prover::Method {
    fn from(value: Method) -> Self {
        match value {
            Method::Get => Self::GET,
            Method::Post => Self::POST,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Params {
    url: String,
    host: Option<String>,
    notary: Option<String>,
    method: Option<Method>,
    headers: Vec<String>,
    data: Option<String>,
    max_sent_data: Option<usize>,
    max_recv_data: Option<usize>,
}

impl TryFrom<Params> for NotarizeParams {
    type Error = anyhow::Error;

    fn try_from(value: Params) -> anyhow::Result<Self> {
        let ProvenUrl { host, port } = parse_proven_url(&value.url)?;

        // If host is not provided fallback to host extracted from url
        let fallback_host = value.host.unwrap_or(host.clone());

        println!("fallback host for notarizing '{fallback_host}'");

        let max_sent_data = value.max_sent_data.unwrap_or(DEFAULT_MAX_SENT_DATA);
        let max_recv_data = value.max_recv_data.unwrap_or(DEFAULT_MAX_RECV_DATA);

        let method = value.method.unwrap_or_else(|| {
            if value.data.is_some() {
                Method::Post
            } else {
                Method::Get
            }
        });

        println!("HTTP method: {method:?}");

        let headers = value
            .headers
            .iter()
            .map(parse_header)
            .collect::<anyhow::Result<Vec<(String, String)>>>()?;

        println!("headers: {headers:#?}");

        let notary_url = value.notary.unwrap_or(DEFAULT_NOTARY_URL.to_string());
        let notary_config = parse_notary_url(&notary_url)?;

        let mut notarize_params_builder = NotarizeParamsBuilder::default();
        notarize_params_builder
            .notary_config(notary_config)
            .server_domain(host)
            .server_host(fallback_host)
            .server_port(port)
            .max_sent_data(max_sent_data)
            .max_recv_data(max_recv_data)
            .uri(value.url)
            .headers(headers)
            .method(method);

        if let Some(body) = value.data {
            notarize_params_builder.body(body);
        }

        Ok(notarize_params_builder.build()?)
    }
}

#[derive(Debug)]
struct ValidatedUrl {
    url: Url,
    host: String,
    scheme: Scheme,
    port: u16,
}

impl ValidatedUrl {
    fn try_from_url(url_str: &str, allowed_schemes: &[Scheme]) -> anyhow::Result<Self> {
        let url =
            Url::parse(url_str).map_err(|_| anyhow::anyhow!("invalid url format: {url_str}"))?;
        let scheme = Scheme::from_str(url.scheme())
            .map_err(|_| anyhow::anyhow!("invalid url protocol: {}", url.scheme()))?;
        if !allowed_schemes.contains(&scheme) {
            anyhow::bail!("invalid url protocol: {}", url.scheme());
        }
        let host = url
            .host_str()
            .ok_or_else(|| anyhow::anyhow!("missing host in url: {url_str}"))?
            .to_string();
        let port = url.port().unwrap_or(match scheme {
            Scheme::Https => 443,
            Scheme::Http => 80,
        });

        Ok(Self {
            url,
            host,
            scheme,
            port,
        })
    }
}

#[derive(Debug)]
struct ProvenUrl {
    host: String,
    port: u16,
}

fn parse_proven_url(url_str: &str) -> anyhow::Result<ProvenUrl> {
    println!("parsing url to notarize '{url_str}'");

    // Only https is allowed for proven urls as it does not make sense to prove http urls (not tls => no tlsn)
    let ValidatedUrl { host, port, .. } = ValidatedUrl::try_from_url(url_str, &[Scheme::Https])?;

    let url = ProvenUrl { host, port };

    println!("proven url: {url:#?}");

    Ok(url)
}

fn parse_notary_url(url_str: &str) -> anyhow::Result<NotaryConfig> {
    println!("parsing notary url '{url_str}'");

    let ValidatedUrl {
        url,
        host,
        scheme,
        port,
    } = ValidatedUrl::try_from_url(url_str, &[Scheme::Https, Scheme::Http])?;

    let path_prefix = url.path().trim_matches('/');
    let enable_tls = scheme == Scheme::Https;

    let config = NotaryConfigBuilder::default()
        .host(host)
        .port(port)
        .path_prefix(path_prefix)
        .enable_tls(enable_tls)
        .build()?;

    println!("notary config: {config:#?}");

    Ok(config)
}

fn parse_header(header_str: impl AsRef<str>) -> anyhow::Result<(String, String)> {
    header_str
        .as_ref()
        .split_once(':')
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .ok_or(anyhow::anyhow!("invalid header format: {}", header_str.as_ref()))
        .and_then(|(key, value)| {
            (!key.is_empty() && !value.is_empty())
                .then_some((key, value))
                .ok_or(anyhow::anyhow!("invalid header format: {}", header_str.as_ref()))
        })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await?;
    Ok(())
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if req.method() != http::Method::POST {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(().into())?);
    }
    let headers = req.headers();
    if let Some(content_type) = headers.get("Content-Type") {
        if content_type != "application/json" {
            return bad_request("wrong content-type");
        }
    } else {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(().into())?);
    }
    let body = req.into_body();
    let params: Params = match serde_json::from_slice(&body) {
        Ok(params) => params,
        Err(err) => return bad_request(format!("invalid params: {err:?}")),
    };
    println!("{params:?}");

    let presentation = match webproof_fetch(params).await {
        Ok(presentation) => presentation,
        Err(err) => return bad_request(format!("web-proof failed: {err:?}")),
    };
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "presentation": presentation
            })
            .to_string()
            .into(),
        )?;
    Ok(response)
}

fn bad_request(msg: impl AsRef<str>) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("content-type", "application/json")
        .body(json!({"error": msg.as_ref()}).to_string().into())?)
}

async fn webproof_fetch(params: Params) -> anyhow::Result<String> {
    let not_params: NotarizeParams = params.try_into()?;
    println!("notarizing...");

    let presentation = generate_web_proof(not_params).await?;

    println!("{presentation}");

    Ok(presentation)
}
