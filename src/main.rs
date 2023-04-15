use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum MessageType {
    Init,
    InitOk,
    Echo,
    EchoOk,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct Body {
    #[serde(rename = "type")]
    ty: MessageType,
    msg_id: usize,
    in_reply_to: Option<usize>,
    echo: String,
}

fn read() -> Option<Message> {
    // Create a mutable buffer to hold the input
    let mut input = String::new();

    // Read input from stdin
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Serialize input to JSON
            let message =
                serde_json::from_str::<Message>(&input).expect("Failed to serialize to JSON");
            Some(message)
        }
        Err(error) => {
            panic!("Failed to read input: {}", error);
        }
    }
}

fn write(msg: Message) {
    let d = serde_json::to_string(&msg).unwrap();
    // Write JSON to stdout
    if let Err(error) = std::io::stdout().write_all(d.as_bytes()) {
        eprintln!("Failed to write to stdout: {}", error);
    }
    // Flush stdout to ensure data is written immediately
    std::io::stdout().flush().expect("Failed to flush stdout");
}

fn main() {
    let message = read().unwrap();
    write(message);
}
