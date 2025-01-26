use async_trait::async_trait;
use pumpkin::{
    command::{
        args::ConsumedArgs, dispatcher::CommandError, tree::CommandTree, tree_builder::require,
        CommandExecutor, CommandSender,
    },
    server::Server,
};

use crate::utils::todo_message;

const NAMES: [&str; 2] = ["vanish", "v"];
const DESCRIPTION: &str = "Vanish from the server.";

struct VanishExecutor;

#[async_trait]
impl CommandExecutor for VanishExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        _: &Server,
        _: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let player = sender.as_player().unwrap();
        player.send_system_message(&todo_message()).await;

        Ok(())
    }
}

pub fn init_command() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(require(|sender| sender.is_player()).execute(VanishExecutor))
}
