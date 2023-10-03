use hyper::{body::to_bytes, Body, Request, Response};
use xin::{
    chat::{
        ChatCompletionResponse, ChatCompletionResponseChoice, ChatCompletionResponseMessage,
        ChatCompletionRole, ChatMessageFunctionCall, FinishReason,
    },
    common::Usage,
};

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
    if req.method().eq(&hyper::http::Method::OPTIONS) {
        println!("*** empty request, return empty response ***");

        let response = Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "*")
            .header("Access-Control-Allow-Headers", "*")
            .body(Body::empty())
            .unwrap();

        return Ok(response);
    }
    let body_bytes = to_bytes(req.body_mut()).await?;

    // ! debug
    let s = std::str::from_utf8(&body_bytes).unwrap();
    dbg!(s);

    let mut chat_request: xin::chat::ChatCompletionRequest =
        serde_json::from_slice(&body_bytes).unwrap();

    dbg!(&chat_request);

    // * improve prompt ======>
    let mut system_prompt = String::new();
    if chat_request.messages[0].role == ChatCompletionRole::System {
        system_prompt = format!(
            "<<SYS>>\n{content} <</SYS>>\n\n",
            content = chat_request.messages[0].content.as_str()
        );
        chat_request.messages.remove(0);
    };

    let user_message = chat_request.messages[0].content.as_str().trim();

    let mut prompt = String::new();
    prompt = format!("<s>[INST] {} {} [/INST]", system_prompt, user_message,);

    dbg!(&prompt);

    // * <======

    // let prompt = chat_request.messages[0].content.as_str();

    // dbg!(prompt);

    // println!("\n*** [prompt begin] ***");
    // println!("{}", prompt);
    // println!("*** [prompt end] ***\n\n");

    {
        // let content = "There's a llama in my garden 😱 What should I do?";

        // let system_prompt = r###"You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe.  Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.

        // If a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don't know the answer to a question, please don't share false information."###;

        // let system_prompt = r###"You are a helpful, respectful and honest assistant."###;

        // let user_prompt = r###"What's the capital of France?"###;

        // let prompt = format!(
        //     r###"
        // <s>[INST] <<SYS>>
        // {system_prompt}
        // <</SYS>>

        // {user_prompt} [/INST]"###,
        //     system_prompt = system_prompt,
        //     user_prompt = user_prompt,
        // );

        // dbg!(&prompt);

        // let prompt = r###"
        // <s>[INST] <<SYS>>
        // You are a helpful, respectful and honest assistant.
        // <</SYS>>

        // What's the capital of France? [/INST]  Ah, a question about the beautiful country of France! *adjusts glasses* The capital of France is none other than Paris, my dear. 🇫🇷 It's a city known for its stunning architecture, art museums, and romantic atmosphere. Have you ever been there?</s><s>[INST] How can I get there? Swim or walk? [/INST]"###;

        //     let prompt = r###"
        //     <s>[INST] <<SYS>>
        // You are a helpful, respectful and honest assistant.
        // <</SYS>>

        // What's the capital of France? [/INST] Of course! The capital of France is Paris. 🇫🇷 It's a beautiful city known for its iconic landmarks like the Eiffel Tower, Notre-Dame Cathedral, and the Louvre Museum. 😊 Is there anything else I can help you with?</s><s>[INST] How can I get there? [/INST]"###;
    }

    let buffer = infer(model_name.as_ref(), &prompt).await;
    let model_answer = String::from_utf8(buffer.clone()).unwrap();
    let assistant_message = model_answer.trim();

    dbg!(assistant_message);

    // prepare ChatCompletionResponse
    let chat_completion_obejct = ChatCompletionResponse {
        id: String::new(),
        object: String::from("chat.completion"),
        created: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        model: chat_request.model.clone(),
        choices: vec![ChatCompletionResponseChoice {
            index: 0,
            message: ChatCompletionResponseMessage {
                role: ChatCompletionRole::Assistant,
                content: String::from(assistant_message),
                function_call: None,
            },
            finish_reason: FinishReason::stop,
        }],
        usage: Usage {
            prompt_tokens: 9,
            completion_tokens: 12,
            total_tokens: 21,
        },
    };

    let response = Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        // .body(Body::from(buffer))
        .body(Body::from(
            serde_json::to_string(&chat_completion_obejct).unwrap(),
        ))
        .unwrap();

    println!("============ End of one-turn chat ============\n\n");

    Ok(response)

    // let buffer = infer(model_name.as_ref(), &prompt).await;

    // Ok(Response::new(Body::from(buffer)))
}

pub(crate) async fn infer(model_name: impl AsRef<str>, prompt: impl AsRef<str>) -> Vec<u8> {
    let graph =
        wasi_nn::GraphBuilder::new(wasi_nn::GraphEncoding::Ggml, wasi_nn::ExecutionTarget::CPU)
            .build_from_cache(model_name.as_ref())
            .unwrap();
    // println!("Loaded model into wasi-nn with ID: {:?}", graph);

    let mut context = graph.init_execution_context().unwrap();
    // println!("Created wasi-nn execution context with ID: {:?}", context);

    let tensor_data = prompt.as_ref().trim().as_bytes().to_vec();
    // println!("Read input tensor, size in bytes: {}", tensor_data.len());
    context
        .set_input(0, wasi_nn::TensorType::U8, &[1], &tensor_data)
        .unwrap();

    // Execute the inference.
    context.compute().unwrap();
    // println!("Executed model inference");

    // Retrieve the output.
    let mut output_buffer = vec![0u8; 2048];
    let size = context.get_output(0, &mut output_buffer).unwrap();

    output_buffer[..size].to_vec()
}
