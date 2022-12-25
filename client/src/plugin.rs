use crate::client_is_live;
use actions::plugin::ActionsPlugin;
use airlocks::plugin::AirLocksPlugin;
use bevy::{
    prelude::{App, CorePlugin, Plugin, PluginGroup, TaskPoolOptions},
    window::{PresentMode, WindowDescriptor, WindowMode, WindowPlugin, WindowPosition},
    winit::WinitSettings,
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use chat::plugin::ChatPlugin;
use computers::plugin::ComputersPlugin;
use console_commands::plugins::ConsoleCommandsPlugin;
use construction_tool_admin::plugin::ConstructionToolAdminPlugin;
use controller::plugin::ControllerPlugin;
use counter_windows::plugin::CounterWindowsPlugin;
use entity::plugin::EntityPlugin;
use gridmap::plugin::GridmapPlugin;
use helmet_security::plugin::HelmetsPlugin;
use human_male::plugin::HumanMalePlugin;
use inventory::plugin::InventoryPlugin;
use jumpsuit_security::plugin::JumpsuitsPlugin;
use line_arrow::plugin::LineArrowPlugin;
use main_menu::plugin::MainMenuPlugin;
use map::plugin::MapPlugin;
use networking::plugin::NetworkingPlugin;
use pawn::plugin::PawnPlugin;
use physics::plugin::PhysicsPlugin;
use pistol_l1::plugin::PistolL1Plugin;
use player::plugin::PlayerPlugin;
use point_light::plugin::PointLightPlugin;
use resources::{core::ClientInformation, plugin::ResourcesPlugin};
use setup_ui::plugin::SetupUiPlugin;
use sfx::plugin::SfxPlugin;
use sounds::plugin::SoundsPlugin;
use ui::plugin::UiPlugin;
/// The main plugin to add to execute the client.
pub struct ClientPlugin {
    pub version: String,
    pub threads_amount: Option<u8>,
}

impl Default for ClientPlugin {
    fn default() -> Self {
        Self {
            version: "0.0.0".to_string(),
            threads_amount: None,
        }
    }
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        let task_pool;
        match self.threads_amount {
            Some(amn) => {
                task_pool = TaskPoolOptions::with_num_threads(amn.into());
            }
            None => task_pool = TaskPoolOptions::default(),
        }

        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Space Frontiers ".to_string() + &self.version,
                        width: 1280.,
                        height: 720.,
                        present_mode: PresentMode::AutoNoVsync,
                        position: WindowPosition::Centered,
                        mode: WindowMode::Windowed,
                        transparent: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(CorePlugin {
                    task_pool_options: task_pool,
                }),
        )
        .insert_resource(WinitSettings::game())
        .add_plugin(NetworkingPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(EguiPlugin)
        .insert_resource(ClientInformation {
            version: self.version.clone(),
        })
        .add_plugin(UiPlugin)
        .add_plugin(SetupUiPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ActionsPlugin)
        .add_plugin(ChatPlugin)
        .add_plugin(ConsoleCommandsPlugin)
        .add_plugin(ControllerPlugin::default())
        .add_plugin(EntityPlugin)
        .add_plugin(GridmapPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PawnPlugin)
        .add_plugin(SfxPlugin)
        .add_plugin(ResourcesPlugin)
        .add_plugin(HumanMalePlugin)
        .add_plugin(ConstructionToolAdminPlugin)
        .add_plugin(AirLocksPlugin)
        .add_plugin(CounterWindowsPlugin)
        .add_plugin(ComputersPlugin)
        .add_plugin(JumpsuitsPlugin)
        .add_plugin(HelmetsPlugin)
        .add_plugin(PistolL1Plugin)
        .add_plugin(LineArrowPlugin)
        .add_plugin(PointLightPlugin)
        .add_plugin(SoundsPlugin)
        .add_startup_system(client_is_live);
    }
}
