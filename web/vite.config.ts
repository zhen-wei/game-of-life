import UnoCSS from 'unocss/vite';
import { defineConfig } from 'vite';
// import wasm from "vite-plugin-wasm";

export default defineConfig({
    plugins: [ UnoCSS()],
    base: './',
    server: {
        headers: {
            'Cross-Origin-Opener-Policy': 'same-origin',
            'Cross-Origin-Embedder-Policy': 'require-corp',
        },
        fs: {
            allow: ['.', '../pkg'],
        },
    },
    build: {
        reportCompressedSize: false,
    },
    worker: {
        format: 'es',
    }
});
