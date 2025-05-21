use kalosm::language::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = Llama::new_chat().await?;

    // New code
    let mut chat = model
        .chat()
        .with_system_prompt("The assistant will act like a pirate");

    loop {
        chat(&prompt_input("\n> ")?).to_std_out().await?;
    }
}