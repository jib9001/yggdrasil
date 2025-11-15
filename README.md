# Yggdrasil

Yggdrasil is a configurable 3D raycasting engine written in Rust, utilizing SDL2 and OpenGL for real-time rendering. It creates a first-person 3D perspective from a 2D grid-based world, similar to classic games like Wolfenstein 3D and Doom, but with modern graphics libraries and flexible configuration options.

## Features

### Core Raycasting Engine
- **Fisheye Correction**: Proper perspective projection for realistic wall rendering
- **Real-time 3D Rendering**: Converts 2D map data into a fully 3D first-person view
- **Configurable Resolution**: Adjustable render resolution

### Flexible Configuration System
- **Scalable Field of View**: Choose from predefined FOV options or set custom values:
  - `Narrow` (45°) - Zoomed in view for precision
  - `Normal` (60°) - Balanced standard perspective
  - `Wide` (90°) - Wider field of view
  - `UltraWide` (120°) - Very wide, cinematic view
  - `Custom(f32)` - Any custom FOV in radians
- **Adjustable Ray Count**: Configurable ray density 
- **Automatic Scaling**: Ray spacing automatically adjusts to maintain FOV coverage

### Dual Rendering Pipeline
- **3D Raycasted View**: 3D perspective rendering
- **2D Debug View**: Overhead map view showing player, rays, and map layout
- **Split-Screen Layout**: Both views displayed simultaneously for development and debugging

### Advanced Graphics
- **OpenGL 4.1 Integration**: Modern graphics pipeline with "custom" shaders
- **Texture-based Rendering**: Raycasted scene rendered to texture for efficient scaling
- **Nearest-Neighbor Filtering**: Crisp, pixel-perfect upscaling maintains retro aesthetic
- **Custom GLSL Shaders**: Separate shader programs for geometry and texture rendering
- **Real-time Buffer Management**: Dynamic vertex array and texture buffer updates

### Player System
- **Smooth Movement**: WASD controls with real-time position updates
- **Analog Rotation**: Smooth directional control with proper angle wrapping
- **Collision-Aware**: Movement system respects map boundaries
- **Visual Representation**: Player and direction indicator shown in 2D view

### Performance & Reliability
- **Safe Array Access**: Bounds-checked indexing prevents crashes with any configuration
- **Optimized Raycasting**: Efficient grid traversal with early termination
- **Cross-Platform**: Works on Linux, macOS, and Windows

## Prerequisites

Before building and running the project, ensure you have the following installed:

### Rust and Cargo

Yggdrasil is written in Rust and uses Cargo as its build system and package manager.

### SDL2

Install SDL2 development libraries:

- **Debian/Ubuntu**:  
  `sudo apt-get install libsdl2-dev`
- **Fedora**:  
  `sudo dnf install SDL2-devel`
- **Arch Linux**:  
  `sudo pacman -S sdl2`
- **OpenSUSE**:  
  `sudo zypper install libSDL2-devel`
- **macOS**:  
  `brew install sdl2`
- **Windows**:  
  Download at [official website](https://github.com/libsdl-org/SDL/releases).

### OpenGL Development Libraries

Install OpenGL development libraries:

- **Debian/Ubuntu**:  
  `sudo apt-get install libgl1-mesa-dev`
- **Fedora**:  
  `sudo dnf install mesa-libGL-devel`
- **Arch Linux**:  
  `sudo pacman -S mesa`
- **OpenSUSE**:  
  `sudo zypper install Mesa-libGL-devel`
- **macOS**:  
  OpenGL is included by default.
- **Windows**:  
  OpenGL should be included in graphics driver.

### X11 Extension Headers (Linux only)

- **Debian/Ubuntu**:  
  `sudo apt-get install libxext-dev`
- **Fedora**:  
  `sudo dnf install libXext-devel`
- **Arch Linux**:  
  `sudo pacman -S libxext`
- **OpenSUSE**:  
  `sudo zypper install libXext-devel`

## Building the Project

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd yggdrasil
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the project:
   ```bash
   cargo run
   ```

## Configuration

You can easily customize the engine's behavior by modifying constants in `src/window_gl.rs`:

### Field of View Settings
```rust
// Change the field of view
pub const CURRENT_FOV: FieldOfView = FieldOfView::Normal;   // 60° standard
pub const CURRENT_FOV: FieldOfView = FieldOfView::Wide;     // 90° wide
pub const CURRENT_FOV: FieldOfView = FieldOfView::UltraWide; // 120° ultra-wide
pub const CURRENT_FOV: FieldOfView = FieldOfView::Custom(1.57); // Custom (90° in radians)
```

### Rendering Parameters
```rust
pub const RENDER_X: i32 = 120;     // Texture width (affects detail level)
pub const RENDER_Y: i32 = 120;     // Texture height (affects detail level)
pub const RAYS_COUNT: i32 = 100;   // Number of rays cast (affects quality/performance)
```

### Map Configuration
The 8×8 grid map can be modified in `window_gl.rs`:
```rust
pub static MAP: [[u8; MAP_X as usize]; MAP_Y as usize] = [
    [1, 1, 1, 1, 1, 1, 1, 1],  // 1 = wall, 0 = empty space
    [1, 0, 1, 0, 0, 0, 0, 1],
    // ... customize your map layout
];
```

## Project Structure

```
yggdrasil/
├── src/
│   ├── main.rs          # Entry point of the application
│   ├── draw_gl.rs       # OpenGL helper functions for rendering
│   ├── log.rs           # Logging utilities
│   ├── player.rs        # Player struct and movement logic
│   ├── render_gl.rs     # Shader and OpenGL program management, vertex construction
│   ├── square.rs        # Square struct for map tiles
│   ├── window_gl.rs     # SDL2 window and OpenGL context setup, map constants
│   └── shaders/
│       ├── triangle.vert    # Vertex shader for colored geometry
│       ├── triangle.frag    # Fragment shader for colored geometry
│       ├── tex.vert         # Vertex shader for textured canvas
│       └── tex.frag         # Fragment shader for textured canvas
├── Cargo.toml           # Rust project configuration
└── README.md            # Project documentation
```

## Controls

- **W**: Move forward
- **S**: Move backward  
- **A**: Rotate left (counter-clockwise)
- **D**: Rotate right (clockwise)
- **ESC**: Quit application

### Key Files
- **`main.rs`**: Contains the main game loop, input handling, and dual rendering pipeline.
- **`render_gl.rs`**: Core raycasting engine, shader management, and vertex construction.
- **`draw_gl.rs`**: OpenGL utilities for buffer management, texture handling, and rendering primitives.
- **`window_gl.rs`**: Configuration constants, SDL2 window setup, and map data.
- **`player.rs`**: Player entity with movement, rotation, and position management.
- **`square.rs`**: Map tile representation and rendering data.

## How It Works

1. **Raycasting**:

   - Rays are cast from the player's position at different angles.
   - Each ray checks for intersections with walls in the grid-based map.
   - The shortest distance (horizontal or vertical) is used for each column, with fisheye correction.
   - The wall height is calculated and drawn into a 60×60 pixel buffer.

2. **Rendering**:

   - The pixel buffer is uploaded as a texture to OpenGL.
   - A screen-aligned quad (canvas) displays the texture, scaled up with nearest-neighbor filtering for crisp pixels.
   - The map, player, and rays are also rendered as colored geometry for debugging.

3. **Player Movement**:

   - The player can rotate and move forward/backward using WASD keys.
   - The player's position and direction affect the raycasting and rendering.

4. **Shaders**:
   - `triangle.vert`/`triangle.frag`: For colored geometry (map, player, rays).
   - `tex.vert`/`tex.frag`: For rendering the raycasted scene as a texture.

## Screenshots

### Screenshot 1

![Screenshot 1](https://media.discordapp.net/attachments/634540217822478363/1375981119811616788/image.png?ex=6833a9c7&is=68325847&hm=fb4d1e0add5a4ca7319a1d26c61ecee5d1878aad7017846f94ada26bed0a440a&=&format=webp&quality=lossless)

### Screenshot 2

![Screenshot 2](https://media.discordapp.net/attachments/634540217822478363/1375982410155425893/image.png?ex=6833aafb&is=6832597b&hm=896ee725bcd725e1ebd2ea84df2ad0e40f5e82d96f05fb4b8e15954ec71945e6&=&format=webp&quality=lossless)

## Troubleshooting

- If you see only a solid color or no walls, check your OpenGL driver, SDL2 installation, and that your system supports OpenGL 4.1+.
- If you see blurry pixels, ensure nearest-neighbor filtering is set in `draw_gl.rs` (`gl::TexParameteri` with `gl::NEAREST`).
- For debugging, print the contents of the `_pixels` array before uploading the texture.
