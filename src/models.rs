use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Idol {
    #[serde(rename = "idol_id")]
    id: u8,
    #[serde(rename = "idol_name")]
    name: String,
}

impl Idol {
    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Position {
    #[serde(flatten)]
    idol: Idol,
    score: u64,
    rank: u8,
}

impl Position {
    pub fn idol(&self) -> &Idol {
        &self.idol
    }

    pub fn score(&self) -> u64 {
        self.score
    }

    pub fn rank(&self) -> u8 {
        self.rank
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ranking {
    id: u8,
    name: String,
    data: Vec<Vec<Position>>,
}

impl Ranking {
    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data(&self) -> &[Position] {
        &self.data[0]
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Arrangement {
    pub idol_name: String,
    pub role_name: String,
    pub score: u64,
    pub rank: u8,
}

impl Arrangement {
    pub fn new(idol_name: &str, role_name: &str, score: u64, rank: u8) -> Arrangement {
        Arrangement {
            idol_name: idol_name.to_owned(),
            role_name: role_name.to_owned(),
            score: score,
            rank: rank,
        }
    }
}
