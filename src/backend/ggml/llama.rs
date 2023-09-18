use wasi_nn;

pub(crate) fn infer(prompt: impl AsRef<str>) -> Vec<u8> {
    // let args: Vec<String> = env::args().collect();
    let model_name: &str = "default"; // &args[1];

    // let prompt: &str = &args[2];

    let graph =
        wasi_nn::GraphBuilder::new(wasi_nn::GraphEncoding::Ggml, wasi_nn::ExecutionTarget::CPU)
            .build_from_cache(model_name)
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

    // let output = String::from_utf8(output_buffer.clone()).unwrap();
    // println!("Output: {}", output);

    output_buffer
}

pub(crate) async fn llama_models_handler() {
    unimplemented!("llama_models_handler not implemented")
}

pub(crate) async fn llama_embeddings_handler() {
    unimplemented!("llama_embeddings_handler not implemented")
}

pub(crate) async fn llama_completions_handler() {
    unimplemented!("llama_completions_handler not implemented")
}

pub(crate) async fn llama_chat_completions_handler() {
    unimplemented!("llama_chat_completions_handler not implemented")
}
