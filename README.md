# Rust Doom Render Demo

This is a Rust-based demonstration of CPU rendering inspired by the original C code available at [3DSage/OpenGL Doom tutorial](https://github.com/3DSage/OpenGL-Doom_tutorial_part_2/blob/main/Doom_Part_1.c).

## Introduction

This project serves as a learning exercise for Rust programming language. It showcases a basic implementation of a Doom-style renderer using Rust, providing an alternative to the original C code provided in the source repository.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.

## Getting Started

Follow these steps to get started with this project:

1. Clone this repository:

   ```bash
   git clone https://github.com/Gabriele91/rust-doom-demo.git
   cd rust-doom-demo
   ```

2. Build and run the Rust Doom renderer:

   ```bash
   cargo run --release  -- --textures assets/textures --map assets/box.map
   ```

## Usage

To run the Rust Doom renderer, use the following commands:

- Start the application with `cargo run`.
- Use the following controls to navigate:
  - W, A, S, D: Move forward, left, backward, and right.
  - R: Ascend.
  - F: Descend.
  - Arrow keys: Change the view direction.

## Examples

3D map without texture:
![Example](https://github.com/Gabriele91/rust-doom-demo/blob/main/doc/screenshot.png?raw=true)

3D map with texture:
![Example](https://github.com/Gabriele91/rust-doom-demo/blob/main/doc/screenshot_texture.png?raw=true)

## Contributing

If you would like to contribute to this project, please open an issue to discuss potential changes or submit a pull request with your improvements. We welcome contributions from the community.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Original C code: [3DSage/OpenGL-Doom_tutorial_part_2](https://github.com/3DSage/OpenGL-Doom_tutorial_part_2)
