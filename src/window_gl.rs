pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 700;

pub const MAP_X: i32 = 8;
pub const MAP_Y: i32 = 8;
pub const MAP_S: i32 = MAP_X * MAP_Y;
pub const RENDER_X: i32 = 360;
pub const RENDER_Y: i32 = 360;
pub const RAYS_COUNT: i32 = 360;
// Current FOV setting - change this to adjust the field of view
pub const CURRENT_FOV: FieldOfView = FieldOfView::Normal;
pub const FOV: f32 = CURRENT_FOV.to_radians();

// Field of View options
#[derive(Debug, Clone, Copy)]
pub enum FieldOfView {
    Narrow, // 45 degrees - zoomed in view
    Normal, // 60 degrees - standard view
    Wide, // 90 degrees - wide view
    UltraWide, // 120 degrees - very wide view
    Custom(f32), // Custom FOV in radians
}

impl FieldOfView {
    pub const fn to_radians(self) -> f32 {
        match self {
            FieldOfView::Narrow => std::f32::consts::FRAC_PI_4, // 45°
            FieldOfView::Normal => std::f32::consts::FRAC_PI_3, // 60°
            FieldOfView::Wide => std::f32::consts::FRAC_PI_2, // 90°
            FieldOfView::UltraWide => 2.0 * std::f32::consts::FRAC_PI_3, // 120°
            FieldOfView::Custom(radians) => radians,
        }
    }

    pub fn to_degrees(self) -> f32 {
        (self.to_radians() * 180.0) / std::f32::consts::PI
    }
}

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

pub fn single_index_map() -> [u8; MAP_S as usize] {
    let mut new_map = [0; MAP_S as usize];

    for i in 0..MAP_X {
        for ii in 1..=MAP_Y {
            new_map[(i * MAP_X + ii - 1) as usize] = MAP[i as usize][(ii - 1) as usize];
        }
    }

    return new_map;
}
