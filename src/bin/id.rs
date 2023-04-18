use maelstrom::{Node, Server};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use tuid::TuidGenerator;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Message {
    // init
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    // echo
    Generate,
    GenerateOk {
        id: String,
    },
}

struct IdNode {
    generator: RefCell<TuidGenerator>,
}

impl IdNode {
    pub fn new() -> Self {
        let generator = TuidGenerator::new(1).unwrap();
        Self {
            generator: RefCell::new(generator),
        }
    }
}

impl Node for IdNode {
    type Payload = Message;

    fn process(&self, message: &Self::Payload) -> Self::Payload {
        match message {
            Message::Init { .. } => Message::InitOk,
            Message::Generate => {
                let id = self.generator.borrow_mut().next().to_string();
                Message::GenerateOk { id }
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let node = IdNode::new();
    let server = Server::new(node);
    server.listen();
}
