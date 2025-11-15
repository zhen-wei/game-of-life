import UnoCSS from 'unocss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [UnoCSS()],
    base: './',
    server: {
        headers: {
            'Cross-Origin-Opener-Policy': 'same-origin',
            'Cross-Origin-Embedder-Policy': 'require-corp',
        },
    },
    build: {
        reportCompressedSize: false,
    },
    worker: {
        format: 'es',
    },
});
