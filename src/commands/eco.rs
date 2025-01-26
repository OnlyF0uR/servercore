use async_trait::async_trait;
use pumpkin::{
    command::{
        args::{
            arg_players::PlayersArgumentConsumer, arg_simple::SimpleArgConsumer, Arg, ConsumedArgs,
        },
        dispatcher::CommandError,
        tree::CommandTree,
        tree_builder::{argument, literal, require},
        CommandExecutor, CommandSender,
    },
    server::Server,
};

use crate::utils::todo_message;

const NAMES: [&str; 1] = ["eco"];
const DESCRIPTION: &str = "Manage player economy.";

const ARG_PLAYER: &str = "player";
const ARG_AMOUNT: &str = "amount";

struct EcoSetExecutor;

#[async_trait]
impl CommandExecutor for EcoSetExecutor {
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

struct EcoAddExecutor;

#[async_trait]
impl CommandExecutor for EcoAddExecutor {
    async fn execute<'a>(
        &self,
        sender: &mut CommandSender<'a>,
        _: &Server,
        args: &ConsumedArgs<'a>,
    ) -> Result<(), CommandError> {
        let Some(Arg::Players(targets)) = args.get(&ARG_PLAYER) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_PLAYER.into())));
        };

        let Some(Arg::Simple(amount)) = args.get(&ARG_AMOUNT) else {
            return Err(CommandError::InvalidConsumption(Some(ARG_AMOUNT.into())));
        };

        // We need to parse the amount to a f64
        let amount = amount
            .parse::<f64>()
            .map_err(|_| CommandError::InvalidConsumption(Some(ARG_AMOUNT.into())))?;

        // We need to check if the amount is negative
        if amount < 0.0 {
            return Err(CommandError::InvalidConsumption(Some(ARG_AMOUNT.into())));
        }

        // We need to check if the amount is infinite
        if amount.is_infinite() {
            return Err(CommandError::InvalidConsumption(Some(ARG_AMOUNT.into())));
        }

        log::info!("Adding {} to {}n", amount, targets.len());

        let player = sender.as_player().unwrap();
        player.send_system_message(&todo_message()).await;

        Ok(())
    }
}

struct EcoRemoveExecutor;

#[async_trait]
impl CommandExecutor for EcoRemoveExecutor {
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

struct EcoResetExecutor;

#[async_trait]
impl CommandExecutor for EcoResetExecutor {
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

// TODO: Move to a proper consumer instead of SimpleArgConsumer for f64s
pub fn init_command() -> CommandTree {
    CommandTree::new(NAMES, DESCRIPTION).then(
        require(|sender| sender.has_permission_lvl(pumpkin_util::PermissionLvl::Three))
            .then(
                literal("set").then(
                    argument(ARG_PLAYER, PlayersArgumentConsumer)
                        .then(argument(ARG_AMOUNT, SimpleArgConsumer).execute(EcoSetExecutor)),
                ),
            )
            .then(
                literal("add").then(
                    argument(ARG_PLAYER, PlayersArgumentConsumer)
                        .then(argument(ARG_AMOUNT, SimpleArgConsumer).execute(EcoAddExecutor)),
                ),
            )
            .then(
                literal("remove").then(
                    argument(ARG_PLAYER, PlayersArgumentConsumer)
                        .then(argument(ARG_AMOUNT, SimpleArgConsumer).execute(EcoRemoveExecutor)),
                ),
            )
            .then(
                literal("reset")
                    .then(argument(ARG_PLAYER, PlayersArgumentConsumer).execute(EcoResetExecutor)),
            ),
    )
}
