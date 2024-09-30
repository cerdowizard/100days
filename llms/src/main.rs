use kalosm::language::*;

#[tokio::main]
async fn main() {
    process_text().await
}


pub async fn process_text(){
    let model = Llama::new_chat().await
    .unwrap();

    let mut chat = Chat::builder(model)
        .with_system_prompt("The assistant will act like a doctor.")
        .build();

    loop{
        chat.add_message(prompt_input("\n> ").unwrap())
            .to_std_out()
            .await
        .unwrap()
    }
}

