# Yggdrasil

Yggdrasil is a 2D raycasting engine written in Rust, utilizing SDL2 and OpenGL for rendering. It simulates a first-person perspective in a grid-based world, similar to early 3D games like Wolfenstein 3D.

## Features

- **Raycasting Engine**: Casts rays to detect walls and render a pseudo-3D environment.
- **Player Movement**: Move the player around the map using keyboard controls.
- **Dynamic Rendering**: Real-time rendering of walls, player position, and rays.
- **OpenGL Integration**: Uses OpenGL for efficient rendering.
- **SDL2 for Input and Window Management**: Handles user input and window creation.

## Prerequisites

Before building and running the project, ensure you have the following installed:

### Rust
Install Rust via [rustup](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### SDL2
Install SDL2 development libraries:

- **Debian-based systems (Ubuntu, etc.)**:
  ```bash
  sudo apt-get install libsdl2-dev
  ```

- **Fedora**:
  ```bash
  sudo dnf install SDL2-devel
  ```

- **Arch Linux**:
  ```bash
  sudo pacman -S sdl2
  ```

- **OpenSUSE**:
  ```bash
  sudo zypper install libSDL2-devel
  ```

- **macOS**:
  ```bash
  brew install sdl2
  ```

- **Windows**:
  Use [vcpkg](https://github.com/microsoft/vcpkg) or download SDL2 from the [official website](https://www.libsdl.org/).

### OpenGL Development Libraries
Install OpenGL development libraries:

- **Debian-based systems (Ubuntu, etc.)**:
  ```bash
  sudo apt-get install libgl1-mesa-dev
  ```

- **Fedora**:
  ```bash
  sudo dnf install mesa-libGL-devel
  ```

- **Arch Linux**:
  ```bash
  sudo pacman -S mesa
  ```

- **OpenSUSE**:
  ```bash
  sudo zypper install Mesa-libGL-devel
  ```

- **macOS**:
  OpenGL is included by default.

- **Windows**:
  Use [vcpkg](https://github.com/microsoft/vcpkg) to install OpenGL or download the necessary libraries from the [official website](https://www.opengl.org/).

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

The project is organized as follows:

```
yggdrasil/
├── src/
│   ├── main.rs          # Entry point of the application
│   ├── draw_gl.rs       # OpenGL helper functions for rendering
│   ├── log.rs           # Logging utilities
│   ├── player.rs        # Player struct and movement logic
│   ├── render_gl.rs     # Shader and OpenGL program management
│   ├── square.rs        # Square struct for map tiles
│   ├── window_gl.rs     # SDL2 window and OpenGL context setup
├── shaders/
│   ├── triangle.vert    # Vertex shader
│   ├── triangle.frag    # Fragment shader
├── Cargo.toml           # Rust project configuration
└── README.md            # Project documentation
```

### Key Files

- **`main.rs`**: Contains the main game loop, input handling, and rendering logic.
- **`player.rs`**: Defines the `Player` struct and handles player movement and direction.
- **`draw_gl.rs`**: Provides utilities for managing OpenGL buffers and rendering primitives.
- **`render_gl.rs`**: Manages shaders and OpenGL programs.
- **`square.rs`**: Represents individual map tiles as squares.
- **`window_gl.rs`**: Handles SDL2 window creation and OpenGL context initialization.

## How It Works

1. **Raycasting**:
   - Rays are cast from the player's position at different angles.
   - Each ray checks for intersections with walls in the grid-based map.
   - The distance to the nearest wall is used to determine the height of the wall slice rendered on the screen.

2. **Rendering**:
   - The map is rendered as a top-down view with squares representing walls.
   - Rays are visualized as lines extending from the player's position.
   - The player's position and direction are rendered as a triangle.

3. **Player Movement**:
   - The player can rotate left or right to change their viewing angle.
   - Moving forward or backward updates the player's position based on their direction.

## Dependencies

The project uses the following Rust crates:

- **`sdl2`**: For window management and input handling.
- **`gl`**: OpenGL bindings for Rust.

## Shaders

The project includes two shaders:

- **Vertex Shader (`triangle.vert`)**:
  - Handles the transformation of vertex positions.
- **Fragment Shader (`triangle.frag`)**:
  - Handles the coloring of fragments (pixels).

## Acknowledgments

- Inspired by early 3D games like Wolfenstein 3D.
- Built with Rust, SDL2, and OpenGL.

## Troubleshooting

If you encounter any issues:

1. Ensure all dependencies are installed.
2. Run the project with verbose output:
   ```bash
   cargo run --verbose
   ```
3. Check for error messages and consult the Rust documentation or SDL2/OpenGL resources.

If the issue persists, feel free to open an issue in the repository.

## Screenshots

*(Add screenshots of the game here to showcase the rendering and gameplay.)*
