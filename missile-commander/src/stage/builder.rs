use crate::{Stage, MAX_BULLET_ON_GAME, MAX_MISSILE_COUNT_SAME_TIME, MISSILE_SPEED_FACTOR};

pub fn load_stages() -> Vec<Stage> {
    let rookie = Stage::new(
        0,
        MAX_MISSILE_COUNT_SAME_TIME,
        10,
        MISSILE_SPEED_FACTOR,
        MAX_BULLET_ON_GAME,
    );
    let specialist = Stage::new(
        1,
        MAX_MISSILE_COUNT_SAME_TIME + 5,
        15,
        MISSILE_SPEED_FACTOR * 1.5,
        MAX_BULLET_ON_GAME + 1,
    );
    let veteran = Stage::new(
        2,
        MAX_MISSILE_COUNT_SAME_TIME,
        15,
        MISSILE_SPEED_FACTOR * 3.,
        MAX_BULLET_ON_GAME + 2,
    );
    vec![rookie, specialist, veteran]
}
