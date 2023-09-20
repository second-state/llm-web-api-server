use hyper::{body::to_bytes, Body, Request, Response};

pub(crate) async fn infer(model_name: impl AsRef<str>, prompt: impl AsRef<str>) -> Vec<u8> {
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

    let content = "There's a llama in my garden 😱 What should I do?";

    // let system_prompt = r###"You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe.  Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.

    // If a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don't know the answer to a question, please don't share false information."###;

    let system_prompt = r###"You are a helpful, respectful and honest assistant."###;

    let user_prompt = r###"What's the capital of France?"###;

    let prompt = format!(
        r###"
    <s>[INST] <<SYS>>
    {system_prompt}
    <</SYS>>

    {user_prompt} [/INST]"###,
        system_prompt = system_prompt,
        user_prompt = user_prompt,
    );

    dbg!(&prompt);

    // let prompt = r###"
    // <s>[INST] <<SYS>>
    // You are a helpful, respectful and honest assistant.
    // <</SYS>>

    // Hello are you? [/INST]  Hello there! 😊 I'm here to help you with your query. However, I must inform you that there is no such thing as a llama in your garden, as llamas are living creatures that do not exist in gardens. 🐮 It's possible that you may have seen a llama in a zoo or a farm, but not in a garden. 😊 Is there anything else I can help you with?</s><s>[INST] Is it possible the llama in my garden escapes from a zoo? [/INST]"###;

    let buffer = infer(model_name.as_ref(), &prompt).await;

    // ! display the contents of the buffer
    // let model_answer = String::from_utf8(buffer.clone()).unwrap();

    Ok(Response::new(Body::from(buffer)))

    // let start_tag = "[/INST]";
    // let end_tag = "[end of text]";

    // let start_index = text.find(start_tag).unwrap() + start_tag.len();
    // let end_index = text.find(end_tag).unwrap();

    // let inst_text = text.get(start_index..end_index).unwrap();

    // println!("model_answer: {}", inst_text);

    // unimplemented!("llama_chat_completions_handler not implemented")
}
