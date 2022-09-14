use crate::main_state::MainState;
use ggez::graphics::{set_window_title};
use ggez::{event, ContextBuilder, GameResult};
use std::{env, path};

mod collider;
mod constant;
mod fermat;
mod game_assets;
mod input_state;
mod main_state;
mod player;
mod sprite;
mod sprite_builder;
mod sprite_type;

fn main() -> GameResult {
    // Pek çok ggez örneğinde standart olduğu üzere oyuncu, kaya ve mermi gibi nesnelere ait
    // görseller resource klasöründe saklanıyor. Bu klasörü Context'e almak için yerini buluyoruz.
    let resource_folder = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    // Game Context nesnesini inşa ediyoruz. Başlık, boyutlar, asset kaynağını belirtiyoruz.
    let ctx_builder = ContextBuilder::new("Rockstroid", "ggez").add_resource_path(resource_folder);

    let (mut ctx, events_loop) = ctx_builder.build()?;
    set_window_title(&mut ctx, "Rockstroid Beta");

    let game = MainState::new(&mut ctx)?;
    event::run(ctx, events_loop, game);
}
