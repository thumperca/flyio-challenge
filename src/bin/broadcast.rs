use maelstrom::{Node, Server};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Message {
    Init,
    InitOk,
    Broadcast { message: usize },
    BroadcastOk,
    Read,
    ReadOk { messages: Vec<usize> },
    Topology,
    TopologyOk,
}

struct BroadcastNode {
    history: RefCell<Vec<usize>>,
}

impl BroadcastNode {
    pub fn new() -> Self {
        Self {
            history: RefCell::new(Vec::with_capacity(16)),
        }
    }
}

impl Node for BroadcastNode {
    type Payload = Message;

    fn process(&self, message: &Self::Payload) -> Self::Payload {
        match message {
            Message::Init => Message::InitOk,
            Message::Broadcast { message } => {
                self.history.borrow_mut().push(*message);
                Message::BroadcastOk
            }
            Message::Read => {
                let messages = self.history.borrow().clone();
                Message::ReadOk { messages }
            }
            Message::Topology => Message::TopologyOk,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let node = BroadcastNode::new();
    let server = Server::new(node);
    server.listen();
}
