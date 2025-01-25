mod commands;

use commands::vanish::{self};
use pumpkin::plugin::Context;
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::PermissionLvl;

#[plugin_method]
async fn on_load(&mut self, server: &Context) -> Result<(), String> {
    server
        .register_command(vanish::init_command(), PermissionLvl::Three)
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
