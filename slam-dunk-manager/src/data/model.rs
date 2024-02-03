use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Player {
    pub id: u16,
    pub full_name: String,
    pub position: Position,
    pub height: f32,
    pub stats: AverageStat,
    pub energy: f32,
    pub transfer_fee: f32,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "# {:<3} - {:<24} {:<16} {:<6.2} {:<6.2}$ #",
            self.id, self.full_name, self.position, self.height, self.transfer_fee
        )
    }
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
impl Display for AverageStat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "# {:<8} {:<8} {:<8} {:<8} {:<8} {:<8} #",
            "Pnt", "Reb", "Ass", "Blc", "Ste", "Trn"
        )?;
        write!(
            f,
            "# {:<8.2} {:<8.2} {:<8.2} {:<8.2} {:<8.2} {:<8.2} #",
            self.points_avg,
            self.rebounds_avg,
            self.assist_avg,
            self.blocks_avg,
            self.steals_avg,
            self.turnovers_avg
        )
    }
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

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Position::Center => "Center",
            Position::ComboGuard => "Combo Guard",
            Position::Guard => "Guard",
            Position::PowerForward => "Power Forward",
            Position::ShootingGuard => "Shooting Guard",
            Position::SmallForward => "Small Forward",
        };
        write!(f, "{output}")
    }
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
    pub is_active: bool,
}
