mod cache;
mod commands;
mod config;
mod db;
mod events;
mod utils;

use core::panic;
use std::sync::Arc;

use pumpkin::plugin::{Context, EventPriority};
use pumpkin_api_macros::{plugin_impl, plugin_method};
use pumpkin_util::permission::Permission;

async fn register_perms(ctx: &Arc<Context>) -> Result<(), String> {
    let playtime_perm = Permission::new(
        "servercore:playtime.see",
        "Use the playtime command",
        pumpkin_util::permission::PermissionDefault::Op(pumpkin_util::PermissionLvl::Zero),
    );
    ctx.register_permission(playtime_perm).await?;

    // let playtime_see_others_perm = Permission::new(
    //     "servercore:playtime.seeothers",
    //     "See other players' playtime",
    //     pumpkin_util::permission::PermissionDefault::Op(pumpkin_util::PermissionLvl::One),
    // );
    // ctx.register_permission(playtime_see_others_perm).await?;

    let pay_perm = Permission::new(
        "servercore:pay.use",
        "Use the pay command",
        pumpkin_util::permission::PermissionDefault::Op(pumpkin_util::PermissionLvl::Zero),
    );
    ctx.register_permission(pay_perm).await?;

    let balance_perm = Permission::new(
        "servercore:balance.see",
        "Use the balance command",
        pumpkin_util::permission::PermissionDefault::Op(pumpkin_util::PermissionLvl::Zero),
    );

    ctx.register_permission(balance_perm).await?;

    // 1 perms
    let vanish_perm = Permission::new(
        "servercore:vanish.use",
        "Use the vanish command",
        pumpkin_util::permission::PermissionDefault::Op(pumpkin_util::PermissionLvl::One),
    );
    ctx.register_permission(vanish_perm).await?;

    let staffchat_perm = Permission::new(
        "servercore:staffchat.use",
        "Use the staffchat command",
        pumpkin_util::permission::PermissionDefault::Op(pumpkin_util::PermissionLvl::One),
    );
    ctx.register_permission(staffchat_perm).await?;

    // 3 perms
    let setspawn_perm = Permission::new(
        "servercore:setspawn.use",
        "Use the setspawn command",
        pumpkin_util::permission::PermissionDefault::Op(pumpkin_util::PermissionLvl::Three),
    );
    ctx.register_permission(setspawn_perm).await?;

    let saveall_perm = Permission::new(
        "servercore:saveall.use",
        "Use the saveall command",
        pumpkin_util::permission::PermissionDefault::Op(pumpkin_util::PermissionLvl::Three),
    );
    ctx.register_permission(saveall_perm).await?;

    let economy_perm = Permission::new(
        "servercore:economy.use",
        "Use the economy command",
        pumpkin_util::permission::PermissionDefault::Op(pumpkin_util::PermissionLvl::Three),
    );
    ctx.register_permission(economy_perm).await?;

    Ok(())
}

#[plugin_method]
async fn on_load(&mut self, server: Arc<Context>) -> Result<(), String> {
    pumpkin::init_log!();

    if let Err(e) = config::setup_config(&server.get_data_folder()).await {
        panic!("Failed to setup config: {}", e);
    };

    if let Err(e) = db::setup_db(&server.get_data_folder()).await {
        panic!("Failed to setup database: {}", e);
    };

    if let Err(e) = register_perms(&server).await {
        panic!("Failed to register permissions: {}", e);
    };

    // Events
    server
        .register_event(
            Arc::new(events::join::JoinHandler),
            EventPriority::Lowest,
            true,
        )
        .await;
    server
        .register_event(
            Arc::new(events::leave::LeaveHandler),
            EventPriority::Lowest,
            true,
        )
        .await;

    // Commands
    server
        .register_command(
            commands::playtime::init_command(),
            "servercore:playtime.see",
        )
        .await;
    server
        .register_command(commands::pay::init_command(), "servercore:pay.use")
        .await;
    server
        .register_command(commands::balance::init_command(), "servercore:balance.see")
        .await;
    server
        .register_command(commands::vanish::init_command(), "servercore:vanish.use")
        .await;
    server
        .register_command(
            commands::staffchat::init_command(),
            "servercore:staffchat.use",
        )
        .await;
    server
        .register_command(
            commands::setspawn::init_command(),
            "servercore:setspawn.use",
        )
        .await;
    server
        .register_command(commands::saveall::init_command(), "servercore:saveall.use")
        .await;
    server
        .register_command(commands::economy::init_command(), "servercore:economy.use")
        .await;

    Ok(())
}

#[plugin_impl]
pub struct MyPlugin {}

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
