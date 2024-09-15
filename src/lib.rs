use wasm_bindgen::prelude::*;
use js_sys;

const LEVEL1: &str = include_str!("../levels/level1.txt");

#[wasm_bindgen]
pub struct Player {
    y: u8,
}

#[wasm_bindgen]
impl Player {
    pub fn new(y: u8) -> Player {
        Player { y }
    }

    pub fn y(&self) -> u8 {
        self.y
    }
}

#[wasm_bindgen]
pub struct Game {
    player: Player,
    score: i32,
    map: Vec<Vec<u8>>,
    scroll_x: u8,
    ended: bool,  // New field
    started: bool,  // New field
    won: bool,  // New field
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            player: Player::new(1), // Initial player position
            score: 0,
            map: Game::load_map_from_string(LEVEL1),
            scroll_x: 0,
            ended: false,  // Initialize ended flag
            started: false,  // Initialize started flag
            won: false,  // Initialize won flag
        }
    }

    fn load_map_from_string(map_str: &str) -> Vec<Vec<u8>> {
        map_str.lines()
            .map(|line| {
                line.chars().map(|ch| match ch {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!("Invalid character in map file"),
                }).collect()
            })
            .collect()
    }

    fn check_collision(&self) -> bool {
        let player_x = self.scroll_x as usize;
        let player_y = self.player.y as usize;
        
        if player_y < self.height() && player_x < self.width() {
            self.map[player_y][player_x] == 1  // 1 represents '#'
        } else {
            false  // Consider out-of-bounds as no collision
        }
    }

    pub fn update(&mut self) {
        if !self.started || self.ended {
            return;  // Don't update if the game hasn't started or has ended
        }

        // Increase scroll_x
        self.scroll_x += 1;

        if self.check_collision() {
            self.ended = true;
            return;  // End the game if collision detected
        }

        // Update win condition
        if self.scroll_x as usize >= self.width() - 1 {
            self.ended = true;
            self.won = true;  // Set won to true when reaching the last column
            return;
        }
    }

    pub fn get_player(&self) -> Player {
        Player::new(self.player.y())
    }

    pub fn move_up(&mut self) {
        if self.started && !self.ended && self.player.y > 0 {
            self.player.y -= 1;
            if self.check_collision() {
                self.ended = true;
            }
        }
    }

    pub fn move_down(&mut self) {
        if self.started && !self.ended && self.player.y < (self.height() - 1) as u8 {
            self.player.y += 1;
            if self.check_collision() {
                self.ended = true;
            }
        }
    }

    pub fn update_score(&mut self, points: i32) {
        self.score += points;
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    // Updated method to get a specific tile from the map
    pub fn get_tile(&self, row: usize, col: usize) -> u8 {
        self.map[row][col]
    }

    // Updated method to set a specific tile in the map
    pub fn set_tile(&mut self, row: usize, col: usize, value: u8) {
        self.map[row][col] = value;
    }

    pub fn get_map(&self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(&self.map.concat()[..])
    }

    pub fn width(&self) -> usize {
        self.map[0].len()
    }

    pub fn height(&self) -> usize {
        self.map.len()
    }

    pub fn get_visible_map(&self) -> js_sys::Uint8Array {
        let visible_width = self.get_visible_width();
        let visible_height = self.get_visible_height();
        let mut visible_map = vec![0u8; visible_width * visible_height];

        for (i, row) in visible_map.chunks_mut(visible_width).enumerate() {
            for (j, tile) in row.iter_mut().enumerate() {
                let map_x = self.scroll_x as usize + j;
                let map_y = i;
                
                *tile = if map_x < self.width() && map_y < self.height() {
                    self.map[map_y][map_x]
                } else {
                    0 // Return '0' for out-of-bounds cells
                };
            }
        }

        js_sys::Uint8Array::from(&visible_map[..])
    }

    pub fn get_visible_width(&self) -> usize {
        8
    }

    pub fn get_visible_height(&self) -> usize {
        4
    }

    // New method to start the game
    pub fn start(&mut self) {
        self.started = true;
    }

    // New method to check if the game has started
    pub fn is_started(&self) -> bool {
        self.started
    }

    // New method to check if the game has ended
    pub fn is_ended(&self) -> bool {
        self.ended
    }

    // New method to check if the game has been won
    pub fn is_won(&self) -> bool {
        self.won
    }
}
