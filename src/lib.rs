use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    score: i32,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game { score: 0 }
    }

    pub fn update_score(&mut self, points: i32) {
        self.score += points;
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }
}
