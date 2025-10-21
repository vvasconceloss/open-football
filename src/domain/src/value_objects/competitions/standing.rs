use crate::entities::club::ClubId;

#[derive(Debug, PartialEq, Eq)]
pub struct StandingRow {
    pub club_id: ClubId,
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
    pub points: u32,
    pub goals_scored: u32,
    pub goals_conceded: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Standing {
    pub entries: Vec<StandingRow>,
}
