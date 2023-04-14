pub struct GameStatistics {
    pub score: u32,
    pub time: f64,
    pub is_player_moving: bool,
    pub ready_for_fire: bool,
    pub is_gameover: bool,
}

impl GameStatistics {
    pub fn new() -> Self {
        Self {
            score: 0,
            time: 0.0,
            is_player_moving: false,
            ready_for_fire: false,
            is_gameover: false,
        }
    }

    pub fn normal_score_update(&mut self) {
        self.score += 1;
    }

    pub fn bonus_score_update(&mut self) {
        self.score += 10;
    }

    pub fn reset_game(&mut self) {
        self.time = 0.0;
        self.score = 0;
        self.is_gameover = false;
    }
}
