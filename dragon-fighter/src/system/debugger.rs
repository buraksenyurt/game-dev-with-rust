use crate::system::player::Player;
use crate::App;
use bevy::prelude::Plugin;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // Debug modda çalışıyorsak debug etmemizi kolaylaştıran plug-in'i uygulamaya ekliyoruz
        // Ayrıca izlemek istediğimiz Player nesnesini de burada bildiriyoruz
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Player>();
        }
    }
}
