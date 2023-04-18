use maelstrom::{Node, Server};
use serde::{Deserialize, Serialize};

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
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
}

struct EchoNode;

impl Node for EchoNode {
    type Payload = Message;

    fn process(&self, message: &Self::Payload) -> Self::Payload {
        match message {
            Message::Init { .. } => Message::InitOk,
            Message::Echo { echo } => Message::EchoOk { echo: echo.clone() },
            _ => unreachable!(),
        }
    }
}

fn main() {
    let server = Server::new(EchoNode);
    server.listen();
}
