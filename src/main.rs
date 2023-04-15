use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use tuid::TuidGenerator;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Body {
    // Node init
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: usize,
    },
    // challenge 1
    Echo {
        msg_id: usize,
        echo: String,
    },
    EchoOk {
        msg_id: usize,
        in_reply_to: usize,
        echo: String,
    },
    // challenge 2
    Generate {
        msg_id: usize,
    },
    GenerateOk {
        id: String,
        msg_id: usize,
        in_reply_to: usize,
    },
    // challenge 3
    Broadcast {
        msg_id: usize,
        message: usize,
    },
    BroadcastOk {
        msg_id: usize,
        in_reply_to: usize,
    },
    Read {
        msg_id: usize,
    },
    ReadOk {
        msg_id: usize,
        in_reply_to: usize,
        messages: Vec<usize>,
    },
    Topology {
        msg_id: usize,
    },
    TopologyOk {
        msg_id: usize,
        in_reply_to: usize,
    },
}

struct Node {
    node_id: usize,
    msg_counter: usize,
    generator: TuidGenerator,
    broadcast_history: Vec<usize>,
}

impl Node {
    pub fn new(node_id: usize) -> Self {
        Self {
            node_id,
            msg_counter: 0,
            generator: TuidGenerator::new(node_id as u8).unwrap(),
            broadcast_history: vec![],
        }
    }

    pub fn next(&mut self, msg: Message) -> Option<Message> {
        let body = match msg.body {
            Body::Init { msg_id, .. } => Some(Body::InitOk {
                in_reply_to: msg_id,
            }),
            Body::Echo { msg_id, echo } => Some(Body::EchoOk {
                msg_id: msg_id + 1,
                in_reply_to: msg_id,
                echo,
            }),
            Body::Generate { msg_id } => {
                let id = self.generator.next();
                Some(Body::GenerateOk {
                    id: id.to_string(),
                    msg_id: msg_id + 1,
                    in_reply_to: msg_id,
                })
            }
            Body::Broadcast { msg_id, message } => {
                self.broadcast_history.push(message);
                Some(Body::BroadcastOk {
                    msg_id: msg_id + 1,
                    in_reply_to: msg_id,
                })
            }
            Body::Read { msg_id } => Some(Body::ReadOk {
                msg_id: msg_id + 1,
                in_reply_to: msg_id,
                messages: self.broadcast_history.clone(),
            }),
            Body::Topology { msg_id } => Some(Body::TopologyOk {
                msg_id: msg_id + 1,
                in_reply_to: msg_id,
            }),
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
