# Game of Life

A high-performance Game of Life demo using Rust + WebAssembly, WebGPU/WebGL rendering, and modern monorepo workflow.

[Page](https://zhen-wei.github.io/game-of-life/)

## Introduction

The project is based on the official introductory tutorial [wasm-game-of-life](https://rustwasm.github.io/docs/book/introduction.html) from Rust WebAssembly, with modifications.

The demo runs WebAssembly (wasm) in a web worker and performs off-screen rendering to prevent blocking the main thread.

![wasm-game-of-life](./img/wasm-game-of-life.jpg)

## Features

- Utilizes WebAssembly and offscreen rendering in a worker for improved performance.
- Using [pixi.js](https://pixijs.com/) for WebGPU/WebGL rendering.
- Try using [unocss](https://unocss.dev/) for styling in small exercises.
- Using [Turbo](https://turborepo.com/) to unify Rust (WASM) and JS builds for faster, simpler workflows.

## Getting Started

```bash

pnpm run dev
```

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
