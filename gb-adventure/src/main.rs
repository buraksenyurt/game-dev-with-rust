mod commandos;
mod common;
mod model;

use crate::commandos::Commando;

fn main() {
    musi_lili::run::<Commando>(musi_lili::load!("Commando Game"));
}
