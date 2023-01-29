use crate::con::ServerMessage;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use tokio::sync::oneshot::Sender;
use uuid::Uuid;

#[derive(Default)]
pub struct BlockManager {
    blocks: Vec<(String, Sender<ServerMessage>)>,
}

impl BlockManager {
    pub fn next_block(&mut self) -> (String, Receiver<ServerMessage>) {
        let (sender, receiver) = oneshot::channel();
        let id = self.next_block_to(sender);
        (id, receiver)
    }

    pub fn next_block_to(&mut self, target: Sender<ServerMessage>) -> String {
        let id = Uuid::new_v4().to_string();
        self.blocks.push((id.clone(), target));
        id
    }

    pub fn answer(&mut self, response: ServerMessage) -> Result<(), ServerMessage> {
        if let Some(command_id) = response.command_id() {
            for i in 0..self.blocks.len() {
                if self.blocks[i].0 == command_id {
                    let sender = self.blocks.swap_remove(i).1;
                    return sender.send(response);
                }
            }
        }
        Err(response)
    }

    pub fn unblock(&mut self, id: &str) {
        for i in 0..self.blocks.len() {
            if self.blocks[i].0 == id {
                self.blocks.swap_remove(i);
            }
        }
    }
}
