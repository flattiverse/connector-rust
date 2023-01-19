use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;
use tokio::sync::oneshot::Receiver;
use uuid::Uuid;
use crate::con::CommandResponse;

#[derive(Default)]
pub struct BlockManager {
    blocks: Vec<(String, Sender<CommandResponse>)>,
}

impl BlockManager {
    pub fn next_block(&mut self) -> (String, Receiver<CommandResponse>) {
        let id = Uuid::new_v4().to_string();
        let (sender, receiver) = oneshot::channel();
        self.blocks.push((id.clone(), sender));
        (id, receiver)
    }

    pub fn answer(&mut self, response: CommandResponse) -> Result<(), CommandResponse> {
        for i in 0..self.blocks.len() {
            if self.blocks[i].0 == response.id() {
                let sender = self.blocks.swap_remove(i).1;
                return sender.send(response);
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