use async_trait::async_trait;
use pumpkin::{
    command::{
        args::ConsumedArgs, dispatcher::CommandError, tree::CommandTree, tree_builder::require,
        CommandExecutor, CommandSender,
    },
    server::Server,
};
use pumpkin_util::text::{color::RGBColor, TextComponent};

const NAMES: [&str; 1] = ["vanish"];
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
        if sender.is_console() {
            sender
                .send_message(TextComponent::text(
                    "This command can only be executed by players.",
                ))
                .await;
            return Ok(());
        }

        let player = sender.as_player().unwrap();
        player
            .send_system_message(
                &TextComponent::text("Function not yet implemented")
                    .color_rgb(RGBColor::new(255, 46, 105)),
            )
            .await;

        Ok(())
    }
}

pub fn init_command() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION)
        .then(require(|sender| sender.is_player()).execute(VanishExecutor))
}
