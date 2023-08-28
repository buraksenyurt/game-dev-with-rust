use crate::game::live_data::resources::LiveData;
use bevy::prelude::*;

pub fn prepare_live_data(mut commands: Commands) {
    commands.insert_resource(LiveData::default());
}

pub fn remove_live_data(mut commands: Commands) {
    commands.remove_resource::<LiveData>();
}

pub fn update_live_data(live_data: ResMut<LiveData>) {
    if live_data.is_changed() {
        info!("Ortam verileri değişti");
    }
}
