use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PlayerData {
    pub requested_username: String,
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PlayerDataList {
    pub data: Vec<PlayerData>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BadgeAward {
    pub badge_id: i64,
    pub awarded_date: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BadgeAwardList {
    pub data: Vec<BadgeAward>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tower {
    pub full_name: String,
    pub acronym: String,
    pub difficulty: f64,
    pub realm: String,
    pub time: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Realm {
    pub name: String,
    pub tower_points: i64,
    pub difficulty_requirent1: Option<DifficultyRequirement>,
    pub difficulty_requirent2: Option<DifficultyRequirement>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DifficultyRequirement {
    pub difficulty: Difficulty,
    pub amount: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Copy)]
pub enum Difficulty {
    Easy = 1,
    Medium = 2,
    Hard = 3,
    Difficult = 4,
    Challenging = 5,
    Intense = 6,
    Remorseless = 7,
    Insane = 8,
    Extreme = 9,
    Terrifying = 10,
    Catastrophic = 11,
}

impl FromStr for Difficulty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        if s == "easy" {
            Ok(Self::Easy)
        } else if s == "medium" {
            Ok(Self::Medium)
        } else if s == "hard" {
            Ok(Self::Hard)
        } else if s == "difficult" {
            Ok(Self::Difficult)
        } else if s == "challenging" {
            Ok(Self::Challenging)
        } else if s == "intense" {
            Ok(Self::Intense)
        } else if s == "remorseless" {
            Ok(Self::Remorseless)
        } else if s == "insane" {
            Ok(Self::Insane)
        } else if s == "extreme" {
            Ok(Self::Extreme)
        } else if s == "terrifying" {
            Ok(Self::Terrifying)
        } else if s == "catastrophic" {
            Ok(Self::Catastrophic)
        } else {
            Err(())
        }
    }
}

impl ToString for Difficulty {
    fn to_string(&self) -> String {
        match self {
            Difficulty::Easy => "Easy".to_string(),
            Difficulty::Medium => "Medium".to_string(),
            Difficulty::Hard => "Hard".to_string(),
            Difficulty::Difficult => "Difficult".to_string(),
            Difficulty::Challenging => "Challenging".to_string(),
            Difficulty::Intense => "Intense".to_string(),
            Difficulty::Remorseless => "Remorseless".to_string(),
            Difficulty::Insane => "Insane".to_string(),
            Difficulty::Extreme => "Extreme".to_string(),
            Difficulty::Terrifying => "Terrifying".to_string(),
            Difficulty::Catastrophic => "Catastrophic".to_string(),
        }
    }
}

impl Difficulty {
    pub fn new(diff: f64) -> Option<Difficulty> {
        let base = diff.floor();
        match base {
            1.0 => Some(Difficulty::Easy),
            2.0 => Some(Difficulty::Medium),
            3.0 => Some(Difficulty::Hard),
            4.0 => Some(Difficulty::Difficult),
            5.0 => Some(Difficulty::Challenging),
            6.0 => Some(Difficulty::Intense),
            7.0 => Some(Difficulty::Remorseless),
            8.0 => Some(Difficulty::Insane),
            9.0 => Some(Difficulty::Extreme),
            10.0 => Some(Difficulty::Terrifying),
            11.0 => Some(Difficulty::Catastrophic),
            _ => None,
        }
    }
}
