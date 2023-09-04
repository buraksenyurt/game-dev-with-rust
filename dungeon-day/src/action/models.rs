use crate::action::Action;
use crate::board::components::Position;
use crate::board::resources::ActiveBoard;
use crate::utility::Location;
use bevy::prelude::*;

pub struct WalkAction {
    pub entity: Entity,
    pub loc: Location,
}

impl Action for WalkAction {
    fn apply(&self, world: &mut World) -> bool {
        let Some(board)=world.get_resource::<ActiveBoard>() else {return false};
        if !board.tiles.contains_key(&self.loc) {
            return false;
        };
        let Some(mut pos)=world.get_mut::<Position>(self.entity) else {return false};
        pos.value = self.loc;
        true
    }
}
