use eternal_memory::{chat::llm::LLMClient, logger::loghandle::LogHandle};
use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
};

#[tokio::main]
async fn main() {
    let mut log = LogHandle::new("./debug/log.log");
    log.info("INFO Message".to_string());
    let mut key = String::new();
    let _ = OpenOptions::new()
        .read(true)
        .open("./debug/key.txt")
        .unwrap()
        .read_to_string(&mut key);
    let mut ip = String::new();
    let _ = OpenOptions::new()
        .read(true)
        .open("./debug/ip.txt")
        .unwrap()
        .read_to_string(&mut ip);
    let mut llm = LLMClient::new(key.trim().to_string(), ip.trim().to_string());
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
