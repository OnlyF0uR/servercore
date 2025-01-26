mod cache;
mod commands;
mod config;
mod db;
mod events;
mod utils;

use core::panic;

use pumpkin::plugin::{Context, EventPriority};
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::PermissionLvl;

#[plugin_method]
async fn on_load(&mut self, server: &Context) -> Result<(), String> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    if let Err(e) = config::setup_config(&server.get_data_folder()).await {
        panic!("Failed to setup config: {}", e);
    };

    if let Err(e) = db::setup_db(&server.get_data_folder()).await {
        panic!("Failed to setup database: {}", e);
    };

    // Events
    server
        .register_event(events::join::JoinHandler, EventPriority::Lowest, true)
        .await;
    server
        .register_event(events::leave::LeaveHandler, EventPriority::Lowest, true)
        .await;

    // Commands
    server
        .register_command(commands::playtime::init_command(), PermissionLvl::Zero)
        .await;
    server
        .register_command(commands::pay::init_command(), PermissionLvl::Zero)
        .await;
    server
        .register_command(commands::balance::init_command(), PermissionLvl::Zero)
        .await;
    server
        .register_command(commands::vanish::init_command(), PermissionLvl::One)
        .await;
    server
        .register_command(commands::staffchat::init_command(), PermissionLvl::One)
        .await;
    server
        .register_command(commands::setspawn::init_command(), PermissionLvl::Three)
        .await;
    server
        .register_command(commands::saveall::init_command(), PermissionLvl::Three)
        .await;
    server
        .register_command(commands::economy::init_command(), PermissionLvl::Three)
        .await;

    Ok(())
}

#[plugin_impl]
pub struct MyPlugin;

impl MyPlugin {
    pub fn new() -> Self {
        MyPlugin {}
    }
}
impl Default for MyPlugin {
    fn default() -> Self {
        Self::new()
    }
}
