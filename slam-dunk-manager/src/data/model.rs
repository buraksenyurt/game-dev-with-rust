use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Player {
    pub full_name: String,
    pub position: Position,
    pub height: f32,
    pub stats: AverageStat,
    pub energy: f32,
    pub transfer_fee: f32,
}

#[derive(Debug)]
pub struct AverageStat {
    pub points_avg: f32,
    pub rebounds_avg: f32,
    pub assist_avg: f32,
    pub blocks_avg: f32,
    pub steals_avg: f32,
    pub turnovers_avg: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Position {
    Center,
    ComboGuard,
    Guard,
    PowerForward,
    ShootingGuard,
    SmallForward,
}

#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub players: Vec<Player>,
    pub attack_power: f32,
    pub defensive_power: f32,
}

#[derive(Debug)]
pub struct Coach {
    pub nick_name: String,
    pub budget: f32,
    pub team: Team,
    pub total_wins: u16,
    pub total_loss: u16,
}

#[derive(Debug)]
pub struct TransferMarket {
    pub players: Vec<Player>,
    pub total_value: f32,
}

#[derive(Debug)]
pub struct League {
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub teams: Vec<Team>,
    pub transfer_market: TransferMarket,
}
