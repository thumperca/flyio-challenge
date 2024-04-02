use maelstrom::broadcast::BroadcastNode;
use maelstrom::{Node, Server};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

fn main() {
    let node = BroadcastNode::new();
    let server = Server::new(node);
    server.listen();
}
