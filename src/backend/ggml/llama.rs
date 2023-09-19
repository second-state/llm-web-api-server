use hyper::{body::to_bytes, Body, Request, Response};

pub(crate) fn infer(model_name: impl AsRef<str>, prompt: impl AsRef<str>) -> Vec<u8> {
    let graph =
        wasi_nn::GraphBuilder::new(wasi_nn::GraphEncoding::Ggml, wasi_nn::ExecutionTarget::CPU)
            .build_from_cache(model_name.as_ref())
            .unwrap();
    println!("Loaded model into wasi-nn with ID: {:?}", graph);

    let mut context = graph.init_execution_context().unwrap();
    println!("Created wasi-nn execution context with ID: {:?}", context);

    let tensor_data = prompt.as_ref().as_bytes().to_vec();
    println!("Read input tensor, size in bytes: {}", tensor_data.len());
    context
        .set_input(0, wasi_nn::TensorType::U8, &[1], &tensor_data)
        .unwrap();

    // Execute the inference.
    context.compute().unwrap();
    println!("Executed model inference");

    // Retrieve the output.
    let mut output_buffer = vec![0u8; 1000];
    context.get_output(0, &mut output_buffer).unwrap();

    output_buffer
}

pub(crate) async fn llama_models_handler() -> Result<Response<Body>, hyper::Error> {
    unimplemented!("llama_models_handler not implemented")
}

pub(crate) async fn llama_embeddings_handler() -> Result<Response<Body>, hyper::Error> {
    unimplemented!("llama_embeddings_handler not implemented")
}

pub(crate) async fn llama_completions_handler() -> Result<Response<Body>, hyper::Error> {
    unimplemented!("llama_completions_handler not implemented")
}

pub(crate) async fn llama_chat_completions_handler(
    mut req: Request<Body>,
    model_name: impl AsRef<str>,
) -> Result<Response<Body>, hyper::Error> {
    let body_bytes = to_bytes(req.body_mut()).await?;
    let data: xin::chat::ChatCompletionRequest = serde_json::from_slice(&body_bytes).unwrap();

    let content = data.messages[0].content.as_str();
    dbg!(&content);

    let buffer = infer(model_name.as_ref(), content);

    // ! display the contents of the buffer
    let output = String::from_utf8(buffer.clone()).unwrap();
    println!("Output: {}", output);

    unimplemented!("llama_chat_completions_handler not implemented")
}
