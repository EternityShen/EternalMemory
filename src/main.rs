use eternal_memory::{
    chat::llm::LLMClient,
    logger::loghandle::{self, LogHandle},
};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let mut log = LogHandle::new("./debug/log.log");
    log.info("INFO Message".to_string());
    let api_key = String::new();
    let mut llm = LLMClient::new(
        api_key,
        "http://192.168.12.145:8080/v1/chat/completions".to_string(),
    );
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit" {
            break;
        }
        println!("你发送了: {}", input);
        llm.chat_stream(input).await.unwrap();
    }
}
