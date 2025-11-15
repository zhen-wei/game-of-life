# Game of Life

[Page](https://zhen-wei.github.io/game-of-life/)

## Introduction

The project is based on the official introductory tutorial [wasm-game-of-life](https://rustwasm.github.io/docs/book/introduction.html) from Rust WebAssembly, with modifications.

The demo runs WebAssembly (wasm) in a web worker and performs off-screen rendering to prevent blocking the main thread.

![wasm-game-of-life](./img/wasm-game-of-life.jpg)

## Features

- Utilizes WebAssembly and offscreen rendering in a worker for improved performance.
- Using [pixi.js](https://pixijs.com/) for WebGPU/WebGL rendering.
- Try using [unocss](https://unocss.dev/) for styling in small exercises.

## Getting Started

To build the WebAssembly module, use the following command:

```bash
cargo build --release \
&& wasm-bindgen --target web --out-dir pkg ./target/wasm32-unknown-unknown/release/game_of_life.wasm
```

Next, navigate to the web directory, install dependencies, and run the demo:

```bash
cd web
pnpm install
pnpm run dev
```

This will launch the demo on http://localhost:5174/.

Note: Ensure that you have Node.js and pnpm installed on your machine.

## Environment Requirements

Ensure that your browser supports OffscreenCanvas, WebGL, and wasm simd. The following browser versions are recommended:

- Chrome: >= 91
- Safari: >= 17
- Firefox: >= 105

> Can I use: https://caniuse.com/offscreencanvas
> 
> webassembly roadmap: https://webassembly.org/roadmap/

## Known Issues

The current performance of WebGPU implementation is not as good as WebGL, but it has potential for the future.
