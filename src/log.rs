// Import utility functions for converting coordinates and constants for window dimensions
use crate::draw_gl;
use draw_gl::get_x;
use draw_gl::get_y;
use crate::window_gl::HEIGHT;
use crate::window_gl::WIDTH;
use crate::player::Player;

// Logs details about a horizontal ray during raycasting
pub fn log_h_ray(
    mx: i32, // Map x-coordinate of the ray
    my: i32, // Map y-coordinate of the ray
    mp: i32, // Map position index
    dof: i32, // Depth of field (number of steps taken)
    a_tan: f32, // Inverse tangent of the ray angle
    ra: f32, // Ray angle
    rx: f32, // Ray x-coordinate
    ry: f32, // Ray y-coordinate
    xo: f32, // Step size in the x-direction
    yo: f32, // Step size in the y-direction
) {
    println!("================");
    println!("===horizontal===");
    println!("================");
    println!("mx => {}", mx); // Log map x-coordinate
    println!("my => {}", my); // Log map y-coordinate
    println!("mp => {}", mp); // Log map position index
    println!("dof => {}", dof); // Log depth of field
    println!("================");
    println!("aTan => {}", a_tan); // Log inverse tangent of the ray angle
    println!("ra => {}", ra); // Log ray angle
    println!("rx => {}", rx); // Log ray x-coordinate
    println!("ry => {}", ry); // Log ray y-coordinate
    println!("xo => {}", xo); // Log step size in the x-direction
    println!("yo => {}", yo); // Log step size in the y-direction
    println!("================");
    println!("================");
}

// Logs details about a vertical ray during raycasting
pub fn log_v_ray(
    mx: i32, // Map x-coordinate of the ray
    my: i32, // Map y-coordinate of the ray
    mp: i32, // Map position index
    dof: i32, // Depth of field (number of steps taken)
    n_tan: f32, // Negative tangent of the ray angle
    ra: f32, // Ray angle
    rx: f32, // Ray x-coordinate
    ry: f32, // Ray y-coordinate
    xo: f32, // Step size in the x-direction
    yo: f32, // Step size in the y-direction
) {
    println!("================");
    println!("====vertical====");
    println!("================");
    println!("mx => {}", mx); // Log map x-coordinate
    println!("my => {}", my); // Log map y-coordinate
    println!("mp => {}", mp); // Log map position index
    println!("dof => {}", dof); // Log depth of field
    println!("================");
    println!("nTan => {}", n_tan); // Log negative tangent of the ray angle
    println!("ra => {}", ra); // Log ray angle
    println!("rx => {}", rx); // Log ray x-coordinate
    println!("ry => {}", ry); // Log ray y-coordinate
    println!("xo => {}", xo); // Log step size in the x-direction
    println!("yo => {}", yo); // Log step size in the y-direction
    println!("================");
    println!("================");
}

// Logs the vertices being pushed for a ray
pub fn log_ray_vertices(player: &Player, rx: f32, ry: f32) {
    println!("================");
    println!("pushing vertices");
    println!("player x => {}", player.get_player_x(4.0)); // Log player's x-coordinate
    println!("player y => {}", player.get_player_y(4.0)); // Log player's y-coordinate
    println!("ray x => {}", get_x(rx, WIDTH)); // Log ray's x-coordinate in normalized OpenGL space
    println!("ray y => {}", get_y(ry, HEIGHT)); // Log ray's y-coordinate in normalized OpenGL space
    println!("end vertices");
    println!("================");
}

// Logs the player's current position
pub fn log_player_pos(player: &Player) {
    println!("-----------------------");
    println!("player position");
    println!("player x => {}", player.x_pos); // Log player's x-coordinate
    println!("player y => {}", player.y_pos); // Log player's y-coordinate
    println!("-----------------------");
}
