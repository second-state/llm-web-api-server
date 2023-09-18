pub(crate) mod llama;
// pub(crate) mod tokenizer;

use super::ServiceConfig;
use hyper::{Body, Request, Response};

pub(crate) async fn handle_llama_request(
    req: Request<Body>,
    service_config: &ServiceConfig,
) -> Result<Response<Body>, hyper::Error> {
    dbg!(req.uri().path());
    dbg!(&service_config);
    dbg!(&req);

    match service_config.path.as_str() {
        "/llama/v1/chat/completions" => llama::llama_chat_completions_handler().await,
        "/llama/v1/completions" => llama::llama_completions_handler().await,
        "/llama/v1/embeddings" => llama::llama_embeddings_handler().await,
        "/llama/v1/models" => llama::llama_models_handler().await,
        _ => panic!("unsupported path"),
    }

    unimplemented!()
}
