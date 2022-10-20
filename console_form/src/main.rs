use console_engine::{ConsoleEngine, KeyCode};

fn main() {
    let mut engine = ConsoleEngine::init(40, 30, 20).unwrap();

    loop {
        engine.wait_frame();
        engine.clear_screen();

        engine.print(0, 10, "I hate hello world :P");

        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }
        engine.draw();
    }
}
