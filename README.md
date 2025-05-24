# Yggdrasil

Yggdrasil is a 2D raycasting engine written in Rust, utilizing SDL2 and OpenGL for rendering. It simulates a first-person perspective in a grid-based world, similar to early 3D games like Wolfenstein 3D.

## Features

- **Raycasting Engine**: Casts rays to detect walls and render a pseudo-3D environment.
- **Player Movement**: Move and rotate the player using keyboard controls.
- **Dynamic Rendering**: Real-time rendering of walls, player position, and rays.
- **OpenGL Integration**: Uses OpenGL for efficient rendering, including a pixel buffer as a texture.
- **SDL2 for Input and Window Management**: Handles user input and window creation.
- **Custom Shaders**: Uses GLSL shaders for both colored geometry and textured canvas rendering.
- **Nearest-Neighbor Texture Scaling**: Ensures crisp, pixel-perfect upscaling of the raycasted scene.

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
  Use [vcpkg](https://github.com/microsoft/vcpkg) or download SDL2 from the [official website](https://www.libsdl.org/).

### OpenGL Development Libraries

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
  Use [vcpkg](https://github.com/microsoft/vcpkg) or download from the [official website](https://www.opengl.org/).

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

## Controls

- **W**: Move forward
- **S**: Move backward
- **A**: Rotate left
- **D**: Rotate right

## Project Structure

```
yggdrasil/
├── src/
│   ├── main.rs          # Main game loop, input, and rendering logic
│   ├── draw_gl.rs       # OpenGL buffer, texture, and rendering helpers
│   ├── log.rs           # Logging utilities for debugging
│   ├── player.rs        # Player struct and movement logic
│   ├── render_gl.rs     # Shader and OpenGL program management
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

### Key Files

- **`main.rs`**: Main loop, raycasting, input, and rendering.
- **`draw_gl.rs`**: OpenGL buffer and texture management, color utilities.
- **`player.rs`**: Player struct and movement.
- **`render_gl.rs`**: Shader compilation and program management.
- **`square.rs`**: Map tile representation.
- **`window_gl.rs`**: SDL2 window/context setup, map constants.
- **`shaders/`**: GLSL shaders for both colored and textured rendering.

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

## Troubleshooting

- If you see only a solid color or no walls, check your OpenGL driver, SDL2 installation, and that your system supports OpenGL 4.1+.
- If you see blurry pixels, ensure nearest-neighbor filtering is set in `draw_gl.rs` (`gl::TexParameteri` with `gl::NEAREST`).
- For debugging, print the contents of the `_pixels` array before uploading the texture.

## Screenshots

*(Add screenshots of the game here to showcase the rendering and gameplay.)*

## License

MIT License (or specify your license here).

---

**Enjoy exploring and hacking on Yggdrasil!**
