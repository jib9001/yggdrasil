pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 700;

pub const MAP_X: i32 = 8;
pub const MAP_Y: i32 = 8;
pub const MAP_S: i32 = 64;

pub static MAP: [[u8; MAP_X as usize]; MAP_Y as usize] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 1, 1, 0, 1],
    [1, 0, 1, 0, 1, 0, 0, 1],
    [1, 0, 1, 0, 1, 0, 1, 1],
    [1, 0, 1, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 1, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

pub fn single_index_map() -> [u8; 64] {
    let mut new_map = [0; 64];

    for i in 0..MAP_X {
        for ii in 1..=MAP_Y {
            new_map[(i * MAP_X + ii - 1) as usize] = MAP[i as usize][(ii - 1) as usize];
        }
    }

    return new_map;
}
