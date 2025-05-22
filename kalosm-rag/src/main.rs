use kalosm::language::*;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = Llama::builder()
    // Specify a custom model source using a GGUF file
    .with_source(LlamaSource::new(
        FileSource::HuggingFace {
            model_id: "QuantFactory/SmolLM-1.7B-Instruct-GGUF".to_string(),
            revision: "main".to_string(),
            file: "SmolLM-1.7B-Instruct.Q4_K_M.gguf".to_string(),
        },
    ))
    .build()
    .await?;

    let thing = {
        2*2
    };

    println!("This is a thing {thing}");

    foo();
    // New code
    let mut chat = model
        .chat()
        .with_system_prompt("The assistant will act like a pirate");

    loop {
        chat(&prompt_input("\n> ")?).to_std_out().await?;
    }
}

/// This is some doc
fn foo() {
    println!("Hello");
}