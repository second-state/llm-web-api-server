use hyper::{
    http::request::Parts,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server,
};
use std::net::SocketAddr;

mod config;
use config::{load_config, GatewayConfig, ServiceConfig};

// type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub state_thing: String,
}

#[tokio::main]
async fn main() {
    let gateway_config = load_config("config.yml");

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let new_service = make_service_fn(move |_| {
        let config = gateway_config.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                let config = config.clone();
                handle_request(req, config)
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle_request(
    req: Request<Body>,
    config: GatewayConfig,
) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path();
    let service_config = match get_service_config(path.clone(), &config.services) {
        Some(service_config) => service_config,
        None => {
            return not_found();
        }
    };

    let auth_token = format!(
        "Bearer {openai_api_key}",
        openai_api_key = std::env::var("OPENAI_API_KEY").unwrap()
    );

    let (parts, body) = req.into_parts();
    let downstream_req = build_downstream_request(parts, body, service_config, auth_token).await?;

    match forward_request(downstream_req).await {
        Ok(res) => Ok(res),
        Err(e) => {
            dbg!(&e);

            service_unavailable(format!("Failed to connect to downstream service. {:?}", e))
        }
    }
}

fn get_service_config<'a>(path: &str, services: &'a [ServiceConfig]) -> Option<&'a ServiceConfig> {
    services.iter().find(|c| path.starts_with(&c.path))
}

fn not_found() -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::from("404 Not Found"));
    *response.status_mut() = hyper::StatusCode::NOT_FOUND;
    Ok(response)
}

async fn build_downstream_request(
    parts: Parts,
    body: Body,
    service_config: &ServiceConfig,
    auth_token: String,
) -> Result<Request<Body>, hyper::Error> {
    let req = Request::from_parts(parts, body);
    let uri = service_config.target_service.as_str();

    let mut downstream_req_builder = Request::builder().uri(uri).method(req.method());

    // headers
    let headers = downstream_req_builder.headers_mut().unwrap();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", auth_token.as_str().parse().unwrap());

    // body
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
    let downstream_req = downstream_req_builder.body(Body::from(body_bytes)).unwrap();

    Ok(downstream_req)
}

async fn forward_request(mut req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    {
        // let body_bytes = to_bytes(req.body_mut()).await?;
        // let body: xin::chat::ChatCompletionRequest = serde_json::from_slice(&body_bytes).unwrap();
        // let s = serde_json::to_string(&body).unwrap();
        // dbg!(req.uri());
        // dbg!(&s);

        // let auth = format!(
        //     "Authorization: Bearer {openai_api_key}",
        //     openai_api_key = std::env::var("OPENAI_API_KEY").unwrap()
        // );
        // let output = std::process::Command::new("curl")
        //     .args([
        //         "https://api.openai.com/v1/chat/completions",
        //         "-X",
        //         "POST",
        //         "-H",
        //         "Content-Type: application/json",
        //         "-H",
        //         &auth,
        //         "-d",
        //         &s,
        //     ])
        //     .output()
        //     .unwrap();
        // dbg!(output.status);

        // let string = String::from_utf8(output.stdout).unwrap();
        // dbg!(string);
    }

    let conn = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .build();

    let client = Client::builder().build::<_, hyper::Body>(conn);

    match client.request(req).await {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}

fn service_unavailable<T>(reason: T) -> Result<Response<Body>, hyper::Error>
where
    T: Into<Body>,
{
    let mut response = Response::new(reason.into());
    *response.status_mut() = hyper::StatusCode::SERVICE_UNAVAILABLE;
    Ok(response)
}
