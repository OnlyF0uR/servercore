use async_trait::async_trait;
use pumpkin::{
    command::{
        args::ConsumedArgs,
        dispatcher::CommandError,
        tree::{builder::require, CommandTree},
        CommandExecutor, CommandSender,
    },
    server::Server,
};

use crate::utils::todo_message;

const NAMES: [&str; 1] = ["setspawn"];
const DESCRIPTION: &str = "Set the original server spawn.";

struct SetSpawnExecutor;

#[async_trait]
impl CommandExecutor for SetSpawnExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender,
        _: &Server,
        _: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let player = sender.as_player().unwrap();
        player.send_system_message(&todo_message()).await;

        // TODO: This

        Ok(())
    }
}

pub fn init_command() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(require(|sender| sender.is_player()).execute(SetSpawnExecutor))
}
