use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Clone)]
struct Node {
    node_id: usize,
    msg_counter: usize,
}

impl Node {
    pub fn new(node_id: usize) -> Self {
        Self {
            node_id,
            msg_counter: 0,
        }
    }

    pub fn next(&mut self, msg: Message) -> Option<Message> {
        let body = match msg.body.ty {
            MessageType::Init => {
                self.msg_counter += 1;
                Some(Body {
                    ty: MessageType::InitOk,
                    msg_id: self.msg_counter,
                    in_reply_to: Some(msg.body.msg_id),
                    extra: Default::default(),
                })
            }
            MessageType::Echo => {
                self.msg_counter += 1;
                Some(Body {
                    ty: MessageType::EchoOk,
                    msg_id: self.msg_counter,
                    in_reply_to: Some(msg.body.msg_id),
                    extra: HashMap::from([(
                        "echo".to_string(),
                        msg.body.extra.get("echo").unwrap().to_owned().into(),
                    )]),
                })
            }
            _ => None,
        }?;
        Some(Message {
            src: msg.dest,
            dest: msg.src,
            body,
        })
    }
}

fn get_node(node_id: &str) -> Node {
    let id = (&node_id[1..])
        .parse::<usize>()
        .expect("Node ID to be valid");
    Node::new(id)
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
    let mut d = serde_json::to_string(&msg).unwrap();
    d.push_str("\n");
    // Write JSON to stdout
    if let Err(error) = std::io::stdout().write_all(d.as_bytes()) {
        eprintln!("Failed to write to stdout: {}", error);
    }
    // Flush stdout to ensure data is written immediately
    std::io::stdout().flush().expect("Failed to flush stdout");
}

fn main() {
    let mut node_holder = None;
    loop {
        let message = read().unwrap();
        let mut node = node_holder.unwrap_or_else(|| {
            let node = get_node(&message.dest);
            node
        });
        if let Some(msg) = node.next(message) {
            write(msg);
        } else {
            break;
        }
        node_holder = Some(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_id() {
        let node = get_node("n2");
        assert_eq!(node.node_id, 2);
    }
}
