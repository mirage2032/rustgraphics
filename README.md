# OpenGL graphics engine in Rust
![build status](https://github.com/mirage2032/rustgraphics/actions/workflows/build.yml/badge.svg)

This is a simple graphics engine written in Rust using OpenGL.  
It is a work in progress project and there's still a lot of work to be done.

## Features

### Finished
- Model loading using assimp
- Blinn-Phong Lighting
  - directional
  - point
  - spot
- Material shading
- Camera controls
- Entity component system

### To do
- Convert models to binary format for faster loading
- Shadow mapping
- Physics simulation

## Compatability

This project was tested and works on both Linux and Window 11

## How to run

The project is split into 2 parts: the engine and the game.  
In order to run the game, you can do so using the following commands:

```shell
cd game
cargo run 
```