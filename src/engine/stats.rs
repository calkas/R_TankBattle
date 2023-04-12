pub struct GameStatistics {
    pub score: u32,
    pub time: f64,
    pub is_player_moving: bool,
    pub ready_for_fire: bool,
}

impl GameStatistics {
    pub fn new() -> Self {
        Self {
            score: 0,
            time: 0.0,
            is_player_moving: false,
            ready_for_fire: false,
        }
    }

    pub fn normal_score_update(&mut self) {
        self.score += 1;
    }

    pub fn bonus_score_update(&mut self) {
        self.score += 10;
    }

    pub fn reset_game_time(&mut self) {
        self.time = 0.0;
    }

    pub fn reset_score(&mut self) {
        self.score = 0;
    }
}
