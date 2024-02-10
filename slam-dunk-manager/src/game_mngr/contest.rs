use crate::prelude::model::*;
use chrono::{DateTime, Duration, Utc};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

pub fn simulate_match_day(league: &mut League) {
    let mut rng = thread_rng();
    let mut teams_shuffled = league.teams.clone();
    teams_shuffled.shuffle(&mut rng);

    for match_teams in teams_shuffled.chunks_exact_mut(2) {
        if let [home, away] = match_teams {
            let home_team_score =
                (home.attack_power - away.defensive_power + rng.gen_range(2.0..20.0)) as i16;
            let away_team_score =
                (away.attack_power - home.defensive_power + rng.gen_range(2.0..20.0)) as i16;

            home.stats.game_played += 1;
            away.stats.game_played += 1;
            home.stats.points_plus += home_team_score;
            away.stats.points_plus += away_team_score;
            home.stats.points_minus += away_team_score;
            away.stats.points_plus += home_team_score;

            home.stats.diff = home.stats.points_plus - home.stats.points_minus;
            away.stats.diff = away.stats.points_plus - away.stats.points_minus;

            match home_team_score.cmp(&away_team_score) {
                Ordering::Greater => {
                    home.stats.win += 1;
                    away.stats.loss += 1;
                }
                Ordering::Less => {
                    away.stats.win += 1;
                    home.stats.loss += 1;
                }
                Ordering::Equal => {}
            }
        }
    }

    league.teams.iter_mut().for_each(|team| {
        if let Some(updated_team) = teams_shuffled.iter().find(|t| t.name == team.name) {
            *team = updated_team.clone();
        }
    });
}

pub fn create_schedule(teams: &[Team], start_date: DateTime<Utc>) -> Vec<MatchDay> {
    let teams_count = teams.len();
    let mut schedule = Vec::new();

    for round in 0..teams_count - 1 {
        let mut match_day = MatchDay {
            id: (round + 1) as u16,
            competitions: Vec::new(),
        };

        for (idx, team) in teams.iter().enumerate() {
            let opponent_idx = (round + idx) % (teams_count - 1);
            if opponent_idx == idx {
                continue;
            }

            let home = if idx % 2 == 0 {
                team
            } else {
                &teams[opponent_idx]
            };
            let visitor = if idx % 2 == 0 {
                &teams[opponent_idx]
            } else {
                team
            };

            let competition = Competition {
                code: format!("COMP{}-{}", round + 1, idx),
                date: start_date + Duration::weeks(round as i64),
                home: home.clone(),
                visitor: visitor.clone(),
            };
            match_day.competitions.push(competition);
        }

        schedule.push(match_day);
    }

    schedule
}
