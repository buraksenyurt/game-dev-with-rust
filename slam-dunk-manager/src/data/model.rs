pub struct Player {
    pub full_name: String,
    pub position: Position,
    pub height: f32,
    pub stats: AverageStat,
    pub energy: f32,
    pub transfer_fee: f32,
}

pub struct AverageStat {
    pub points_avg: f32,
    pub rebounds_avg: f32,
    pub assist_avg: f32,
    pub blocks_avg: f32,
    pub steals_avg: f32,
    pub turnovers_avg: f32,
}

pub enum Position {
    Center,
    ComboGuard,
    Guard,
    PowerForward,
    ShootingGuard,
    SmallForward,
}

pub struct Team {
    pub name: String,
    pub players: Vec<Player>,
    pub attack_power: f32,
    pub defensive_power: f32,
}
