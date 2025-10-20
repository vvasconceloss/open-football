pub struct ClubFixtureStatistics {
    pub goals: u8,
    pub shots: u8,
    pub fouls: u8,
    pub possession: f32,
    pub shots_on_target: u8,
}

pub struct FixtureResult {
    pub home_stats: ClubFixtureStatistics,
    pub away_stats: ClubFixtureStatistics,
}
