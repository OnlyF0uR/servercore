mod commands;
mod db;

use core::panic;

use commands::{
    saveall, setspawn,
    vanish::{self},
};
use db::setup_db;
use pumpkin::plugin::Context;
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::PermissionLvl;

#[plugin_method]
async fn on_load(&mut self, server: &Context) -> Result<(), String> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    if let Err(e) = setup_db(&server.get_data_folder()).await {
        panic!("Failed to setup database: {}", e);
    };

    server
        .register_command(vanish::init_command(), PermissionLvl::Three)
        .await;

    server
        .register_command(setspawn::init_command(), PermissionLvl::Three)
        .await;

    server
        .register_command(saveall::init_command(), PermissionLvl::Three)
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
