// Import utility functions for converting coordinates
use crate::draw_gl::get_y;
use crate::draw_gl::get_x;
// Import constants for window dimensions
use crate::window_gl::HEIGHT;
use crate::window_gl::WIDTH;
// Import the constant for PI
use std::f32::consts::PI;

// The `Player` struct represents the player in the game world
pub struct Player {
    pub x_pos: f32, // Player's x-coordinate in the world
    pub y_pos: f32, // Player's y-coordinate in the world
    pub player_dir: f32, // Player's direction in radians
    pub x_dir: f32, // Player's x-direction vector
    pub y_dir: f32, // Player's y-direction vector
    pub tl_point: [f32; 3], // Top-left corner of the player's representation
    pub tr_point: [f32; 3], // Top-right corner of the player's representation
    pub bl_point: [f32; 3], // Bottom-left corner of the player's representation
    pub br_point: [f32; 3], // Bottom-right corner of the player's representation
    pub color: [f32; 3], // Color of the player (RGB)
}

// Implementation of the `Player` struct
impl Player {
    // Constructor to create a new `Player` instance
    pub fn new(x: f32, y: f32) -> Player {
        let x_pos = x;
        let y_pos = y;
        let player_dir = 0.0; // Initial direction is 0 radians
        let x_dir = 0.0; // Initial x-direction vector
        let y_dir = 0.0; // Initial y-direction vector

        // Calculate the corners of the player's representation
        let tl_point = [get_x(x, WIDTH), get_y(y, HEIGHT), 0.0];
        let tr_point = [get_x(x + 8.0, WIDTH), get_y(y, HEIGHT), 0.0];
        let bl_point = [get_x(x, WIDTH), get_y(y + 8.0, HEIGHT), 0.0];
        let br_point = [get_x(x + 8.0, WIDTH), get_y(y + 8.0, HEIGHT), 0.0];

        let color = [0.0, 0.0, 1.0]; // Default color is blue

        Player {
            x_pos,
            y_pos,
            player_dir,
            x_dir,
            y_dir,
            tl_point,
            tr_point,
            bl_point,
            br_point,
            color,
        }
    }

    // Update the player's position
    pub fn update_pos(&mut self, x: f32, y: f32) {
        self.update_x_pos(x); // Update x-coordinate
        self.update_y_pos(y); // Update y-coordinate
    }

    // Update the player's x-coordinate
    pub fn update_x_pos(&mut self, x: f32) {
        self.x_pos = x;
        self.set_conrners(); // Recalculate the corners
    }

    // Update the player's y-coordinate
    pub fn update_y_pos(&mut self, y: f32) {
        self.y_pos = y;
        self.set_conrners(); // Recalculate the corners
    }

    // Update the player's direction
    pub fn update_dir(&mut self, mut new_dir: f32) {
        // Normalize the direction to stay within [0, 2π]
        while new_dir > 2.0 * PI {
            new_dir -= 2.0 * PI;
        }
        self.player_dir = new_dir;
    }

    // Update the player's x-direction vector
    pub fn update_x_dir(&mut self, new_dir: f32) {
        self.x_dir = new_dir;
    }

    // Update the player's y-direction vector
    pub fn update_y_dir(&mut self, new_dir: f32) {
        self.y_dir = new_dir;
    }

    // Get the player's current direction
    pub fn get_dir(&self) -> f32 {
        return self.player_dir;
    }

    // Get the player's x-direction vector
    pub fn get_x_dir(&self) -> f32 {
        return self.x_dir;
    }

    // Get the player's y-direction vector
    pub fn get_y_dir(&self) -> f32 {
        return self.y_dir;
    }

    // Get the player's x-coordinate in normalized OpenGL space with an offset
    pub fn get_player_x(&self, offset: f32) -> f32 {
        return get_x(self.x_pos + offset, WIDTH);
    }

    // Get the player's y-coordinate in normalized OpenGL space with an offset
    pub fn get_player_y(&self, offset: f32) -> f32 {
        return get_y(self.y_pos + offset, HEIGHT);
    }

    // Recalculate the corners of the player's representation
    fn set_conrners(&mut self) {
        self.tl_point = [get_x(self.x_pos, WIDTH), get_y(self.y_pos, HEIGHT), 0.0];
        self.tr_point = [get_x(self.x_pos + 8.0, WIDTH), get_y(self.y_pos, HEIGHT), 0.0];
        self.bl_point = [get_x(self.x_pos, WIDTH), get_y(self.y_pos + 8.0, HEIGHT), 0.0];
        self.br_point = [get_x(self.x_pos + 8.0, WIDTH), get_y(self.y_pos + 8.0, HEIGHT), 0.0];
    }
}
