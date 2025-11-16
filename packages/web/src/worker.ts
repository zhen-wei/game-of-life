import init, { Universe } from '@game-of-life/engine';
import type { ColorSource, Renderer } from 'pixi.js';
import { DOMAdapter, WebWorkerAdapter, Application, Graphics, ParticleContainer, Particle } from 'pixi.js';
import type { InitPayload, MsgData, TogglePayload } from './payload.js';
import { createMsgData, MsgDataEnum } from './payload.js';

let universe: Universe;
let tickTimer = 0;

DOMAdapter.set(WebWorkerAdapter);

const app = new Application();

function startTick() {
    clearInterval(tickTimer);
    tickTimer = setInterval(() => universe.tick(), 1000 / 60);
}

function createRectTexture(renderer: Renderer, color: ColorSource, size: number) {
    const graphics = new Graphics().rect(0, 0, size, size).fill(color);
    const texture = renderer.generateTexture(graphics);
    graphics.destroy();
    return texture;
}

async function start(initPayload: InitPayload): Promise<void> {
    const { width, height, canvas, cellSize, deadColor = '#FFFFFF', aliveColor = '#000000' } = initPayload;

    await Promise.all([
        init(),
        app.init({
            canvas: canvas,
            hello: true,
            preference: 'webgpu',
        }),
    ]);

    app.renderer.background.color = deadColor;
    app.renderer.resize((cellSize + 1) * width + 1, (cellSize + 1) * height + 1);

    const maxSize = width * height;
    const texture = createRectTexture(app.renderer, aliveColor, cellSize);

    const sprites = new ParticleContainer({
        isRenderGroup: true,
        dynamicProperties: {
            position: false,
            color: true,
        },
    });
    for (let i = 0; i < maxSize; i++) {
        const row = Math.floor(i / width);
        const col = i % width;
        const cell = new Particle({
            texture: texture,
            x: col * (cellSize + 1) + 1,
            y: row * (cellSize + 1) + 1,
        });
        sprites.addParticle(cell);
    }
    app.stage.addChild(sprites);

    universe = Universe.new(width, height);

    const drawCells = () => {
        const cells = universe.cells();
        for (let i = 0; i < cells.length; i++) {
            sprites.particleChildren[i].color = cells[i] ? 0xffffffff : 0x0;
        }
    };
    app.ticker.add(() => {
        drawCells();
        self.postMessage(createMsgData(MsgDataEnum.render, Math.round(app.ticker.FPS)));
    });
    startTick();
}

function toggle(toggle: TogglePayload) {
    universe.toggle_cell(toggle.row, toggle.col);
}

function pause(value: boolean) {
    if (value) {
        clearInterval(tickTimer);
    } else {
        startTick();
    }
}

self.addEventListener('message', (ev) => {
    const { type, payload } = ev.data as MsgData;
    switch (type) {
        case MsgDataEnum.init: {
            start(payload);
            break;
        }
        case MsgDataEnum.toggle: {
            toggle(payload);
            break;
        }
        case MsgDataEnum.pause: {
            pause(payload);
            break;
        }
    }
});
