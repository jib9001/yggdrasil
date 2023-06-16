use crate::draw_gl;
use draw_gl::get_x;
use draw_gl::get_y;
use crate::window_gl::HEIGHT;
use crate::window_gl::WIDTH;
use crate::player::Player;

pub fn log_h_ray(
    mx: i32,
    my: i32,
    mp: i32,
    dof: i32,
    a_tan: f32,
    ra: f32,
    rx: f32,
    ry: f32,
    xo: f32,
    yo: f32
) {
    println!("================");
    println!("===horizontal===");
    println!("================");
    println!("mx => {}", mx);
    println!("my => {}", my);
    println!("mp => {}", mp);
    println!("dof => {}", dof);
    println!("================");
    println!("aTan => {}", a_tan);
    println!("ra => {}", ra);
    println!("rx => {}", rx);
    println!("ry => {}", ry);
    println!("xo => {}", xo);
    println!("yo => {}", yo);
    println!("================");
    println!("================");
}

pub fn log_v_ray(
    mx: i32,
    my: i32,
    mp: i32,
    dof: i32,
    n_tan: f32,
    ra: f32,
    rx: f32,
    ry: f32,
    xo: f32,
    yo: f32
) {
    println!("================");
    println!("====vertical====");
    println!("================");
    println!("mx => {}", mx);
    println!("my => {}", my);
    println!("mp => {}", mp);
    println!("dof => {}", dof);
    println!("================");
    println!("nTan => {}", n_tan);
    println!("ra => {}", ra);
    println!("rx => {}", rx);
    println!("ry => {}", ry);
    println!("xo => {}", xo);
    println!("yo => {}", yo);
    println!("================");
    println!("================");
}

pub fn log_ray_vertices(player: &Player, rx: f32, ry: f32) {
    println!("================");
    println!("pushing vertices");
    println!("player x => {}", player.get_player_x(4.0));
    println!("player y => {}", player.get_player_y(4.0));
    println!("ray x => {}", get_x(rx, WIDTH));
    println!("ray y => {}", get_y(ry, HEIGHT));
    println!("end vertices");
    println!("================");
}
