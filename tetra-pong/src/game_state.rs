/*
Oyun durum bilgilerini tutan State nesnesi ve implementasyonu
*/
use crate::constant::OCEAN_BLUE;
use crate::scenes::{MainMenuScene, Scene, Transition};
use std::fmt::{Display, Formatter};
use tetra::graphics::Color;
use tetra::window::quit;
use tetra::{graphics, Context, State, TetraError};

pub struct GameState {
    pub scenes: Vec<Box<dyn Scene>>,
}

#[derive(Default)]
pub struct Scoreboard {
    pub p1_point: u32,
    pub p2_point: u32,
}

impl Display for Scoreboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "P1: {},P2: {}", self.p1_point, self.p2_point)
    }
}

impl GameState {
    pub fn new(context: &mut Context) -> tetra::Result<GameState> {
        let initial_scene = MainMenuScene::new(context)?;

        Ok(GameState {
            scenes: vec![Box::new(initial_scene)],
        })
    }
}

impl State for GameState {
    fn draw(&mut self, context: &mut Context) -> Result<(), TetraError> {
        graphics::clear(context, Color::hex(OCEAN_BLUE));
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.draw(context)? {
                Transition::None => {}
                Transition::Push(s) => {
                    self.scenes.push(s);
                }
                Transition::Pop => {
                    self.scenes.pop();
                }
            },
            None => quit(context),
        }

        Ok(())
    }

    fn update(&mut self, context: &mut Context) -> Result<(), TetraError> {
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.update(context)? {
                Transition::None => {}
                Transition::Push(s) => {
                    self.scenes.push(s);
                }
                Transition::Pop => {
                    self.scenes.pop();
                }
            },
            None => quit(context),
        }
        Ok(())
    }
}
