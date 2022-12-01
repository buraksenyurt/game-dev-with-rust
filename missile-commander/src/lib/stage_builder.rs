use crate::{Stage, MAX_MISSILE_COUNT_SAME_TIME, MISSILE_SPEED_FACTOR};

pub fn load_stages() -> Vec<Stage> {
    let rookie = Stage::new(100, MAX_MISSILE_COUNT_SAME_TIME, 10, MISSILE_SPEED_FACTOR);
    let specialist = Stage::new(
        200,
        MAX_MISSILE_COUNT_SAME_TIME + 5,
        20,
        MISSILE_SPEED_FACTOR * 1.5,
    );
    let veteran = Stage::new(
        300,
        MAX_MISSILE_COUNT_SAME_TIME + 10,
        50,
        MISSILE_SPEED_FACTOR * 2.,
    );
    vec![rookie, specialist, veteran]
}
