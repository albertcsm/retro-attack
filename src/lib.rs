use wasm_bindgen::prelude::*;
use js_sys;
use web_sys::console;  // Add this import at the top of the file

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
    bullets: Vec<Bullet>,  // New field to track bullets
    time: u32,  // Changed to u32 for integer frame count
}

#[wasm_bindgen]
pub struct Bullet {
    x: u8,
    y: u8,
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
            bullets: Vec::new(),  // Initialize bullets vector
            time: 0,  // Initialize time to 0 frames
        }
    }

    fn load_map_from_string(map_str: &str) -> Vec<Vec<u8>> {
        map_str.lines()
            .map(|line| {
                line.chars().map(|ch| match ch {
                    '.' => 0,
                    '#' => 1,
                    'r' => 2,
                    _ => panic!("Invalid character in map file"),
                }).collect()
            })
            .collect()
    }

    fn check_collision(&self) -> bool {
        let player_x = self.scroll_x as usize;
        let player_y = self.player.y as usize;
        
        if player_y < self.height() && player_x < self.width() {
            self.map[player_y][player_x] > 0  // 0 represents empty space
        } else {
            false  // Consider out-of-bounds as no collision
        }
    }

    pub fn update(&mut self) {
        if !self.started || self.ended {
            return;  // Don't update if the game hasn't started or has ended
        }

        // Increment time by 1 frame
        self.time += 1;

        // Increase scroll_x only when time is even
        if self.time % 2 == 0 {
            self.scroll_x += 1;
        }

        self.bullets.iter_mut().for_each(|bullet| bullet.x += 1);
        
        // Check for bullet collisions with rockets
        let mut bullets_to_remove = Vec::new();
        for (index, bullet) in self.bullets.iter().enumerate() {
            let bullet_x = bullet.x as usize;
            let bullet_y = bullet.y as usize;
            
            // Check if the bullet is within map bounds
            if bullet_y < self.height() && bullet_x < self.width() {
                // Check if the bullet hit a rocket (tile type 2)
                if self.map[bullet_y][bullet_x] == 2 {
                    // Clear the rocket by setting the tile to empty (0)
                    self.map[bullet_y][bullet_x] = 0;
                    // Mark this bullet for removal
                    bullets_to_remove.push(index);
                    // Optionally, you can update the score here
                    // self.update_score(10); // Add 10 points for destroying a rocket
                }
            }
        }

        // Remove the bullets that hit rockets
        for &index in bullets_to_remove.iter().rev() {
            self.bullets.remove(index);
        }

        let width = self.width() as u8;
        self.bullets.retain(|bullet| {
            bullet.x < width && self.map[bullet.y as usize][bullet.x as usize] != 1
        });

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

    // New method to expose scroll_x
    pub fn get_visible_x(&self) -> u8 {
        self.scroll_x
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

    // New method to fire a bullet
    pub fn fire(&mut self) {
        if self.started && !self.ended {
            let bullet = Bullet {
                x: self.scroll_x + 1,  // Start bullet one position ahead of the player
                y: self.player.y,
            };
            self.bullets.push(bullet);
            
            // Add this line to log the firing event
            console::log_1(&"Bullet fired!".into());
        }
    }

    pub fn bullets(&self) -> js_sys::Array {
        self.bullets.iter().map(|bullet| {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"x".into(), &bullet.x.into()).unwrap();
            js_sys::Reflect::set(&obj, &"y".into(), &bullet.y.into()).unwrap();
            obj
        }).collect::<js_sys::Array>()
    }

    pub fn num_bullets(&self) -> usize {
        self.bullets.len()
    }

    // Updated method to get the current game time in frames
    pub fn get_time(&self) -> u32 {
        self.time
    }
}
