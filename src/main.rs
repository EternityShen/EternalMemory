use eternal_memory::logger::loghandle::LogHandle;

fn main() {
    let mut log = LogHandle::new("./debug/log.log");
    log.info("INFO Message".to_string());

    println!("Hello, world!");
}
