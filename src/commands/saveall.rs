use async_trait::async_trait;
use pumpkin::{
    command::{
        args::ConsumedArgs, dispatcher::CommandError, tree::CommandTree, CommandExecutor,
        CommandSender,
    },
    server::Server,
};
use pumpkin_util::text::TextComponent;

const NAMES: [&str; 1] = ["saveall"];
const DESCRIPTION: &str = "Save all worlds.";

struct SaveallExecutor;

#[async_trait]
impl CommandExecutor for SaveallExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        server: &Server,
        _: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        server.save().await;

        sender
            .send_message(TextComponent::text("Saved all worlds."))
            .await;

        Ok(())
    }
}

pub fn init_command() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).execute(SaveallExecutor)
}
