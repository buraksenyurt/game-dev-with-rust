use crate::main_state::MainState;
use ggez::{conf, event, ContextBuilder, GameResult};
use std::{env, path};

mod constant;
mod fermat;
mod game_assets;
mod main_state;
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
    let ctx_builder = ContextBuilder::new("Rockstroid", "ggez")
        .window_setup(conf::WindowSetup::default().title("Rockstroid Beta"))
        .window_mode(conf::WindowMode::default().dimensions(640., 480.))
        .add_resource_path(resource_folder);

    let (mut ctx, events_loop) = ctx_builder.build()?;

    let game = MainState::new(&mut ctx)?;
    event::run(ctx, events_loop, game);
}
