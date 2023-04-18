use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::marker::PhantomData;
use tuid::TuidGenerator;

pub trait Node {
    type Payload;
    fn process(&self, message: &Self::Payload) -> Self::Payload;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<T> {
    src: String,
    dest: String,
    body: Body<T>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
struct Body<T> {
    msg_id: usize,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: T,
}

pub struct Server<T>
where
    T: Node,
    for<'a> T::Payload: Deserialize<'a> + Serialize,
{
    node: T,
}

impl<T> Server<T>
where
    T: Node,
    for<'a> T::Payload: Deserialize<'a> + Serialize,
{
    pub fn new(node: T) -> Self {
        Self { node }
    }

    pub fn listen(&self) {
        loop {
            let message = match self.read() {
                None => break,
                Some(m) => m,
            };
            let response = self.node.process(&message.body.payload);
            self.write(message, response);
        }
    }

    fn read(&self) -> Option<Message<T::Payload>> {
        // Create a mutable buffer to hold the input
        let mut input = String::new();

        // Read input from stdin
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Serialize input to JSON
                let message = serde_json::from_str::<Message<T::Payload>>(&input)
                    .expect("Failed to serialize to JSON");
                Some(message)
            }
            Err(error) => {
                panic!("Failed to read input: {}", error);
            }
        }
    }

    fn write(&self, message: Message<T::Payload>, response: T::Payload) {
        let output = Message {
            src: message.dest,
            dest: message.src,
            body: Body {
                msg_id: message.body.msg_id + 1,
                in_reply_to: Some(message.body.msg_id),
                payload: response,
            },
        };
        let mut d = serde_json::to_string(&output).unwrap();
        d.push_str("\n");
        // Write JSON to stdout
        if let Err(error) = std::io::stdout().write_all(d.as_bytes()) {
            eprintln!("Failed to write to stdout: {}", error);
        }
        // Flush stdout to ensure data is written immediately
        std::io::stdout().flush().expect("Failed to flush stdout");
    }
}
