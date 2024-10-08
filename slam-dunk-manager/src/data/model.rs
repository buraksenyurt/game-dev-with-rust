use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u16,
    pub full_name: String,
    pub position: Position,
    pub height: f32,
    pub stats: AverageStat,
    pub energy: f32,
    pub transfer_fee: f32,
    pub is_free: bool,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "# {:<02}-{:<24} {:<14} {:<6.2}cm {:<5.2}$",
            self.id, self.full_name, self.position, self.height, self.transfer_fee
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            "\t{:<8} {:<8} {:<8} {:<8} {:<8} {:<8}",
            "Pnt", "Reb", "Ass", "Blc", "Ste", "Trn"
        )?;
        write!(
            f,
            "\t{:<8.2} {:<8.2} {:<8.2} {:<8.2} {:<8.2} {:<8.2}",
            self.points_avg,
            self.rebounds_avg,
            self.assist_avg,
            self.blocks_avg,
            self.steals_avg,
            self.turnovers_avg
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    pub players: Vec<Player>,
    pub attack_power: f32,
    pub defensive_power: f32,
    pub stats: TeamStats,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TeamStats {
    pub game_played: u16,
    pub win: u16,
    pub loss: u16,
    pub points_plus: i16,
    pub points_minus: i16,
    pub diff: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    pub nick_name: String,
    pub budget: f32,
    pub team: Team,
    pub total_wins: u16,
    pub total_loss: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferMarket {
    pub players: Vec<Player>,
    pub total_value: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct League {
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub teams: Vec<Team>,
    pub transfer_market: TransferMarket,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Competition {
    pub code: String,
    pub date: DateTime<Utc>,
    pub home: Team,
    pub visitor: Team,
    pub score: MatchScore,
}

impl Display for Competition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} vs {} @ {}",
            self.code,
            self.home.name,
            self.visitor.name,
            self.date.format("%d/%m/%Y %H:%M")
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchDay {
    pub id: u16,
    pub competitions: Vec<Competition>,
    pub is_played: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MatchScore {
    pub home: u16,
    pub visitor: u16,
    pub winner: Winner,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Winner {
    Home,
    Visitor,
    #[default]
    NotPlayed,
}
