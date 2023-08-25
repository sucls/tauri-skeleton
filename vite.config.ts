import {sveltekit} from '@sveltejs/kit/vite';
import {defineConfig} from 'vitest/config';
import { resolve, join } from 'path';

export default defineConfig({
    plugins: [sveltekit()],
    server: {
        hmr: {
            overlay: false // window is not defined ?
        },
    },
    resolve: {
        alias:{
            '$lib': resolve(__dirname, 'src/lib'),
            '@': join(__dirname, 'src'),
        }
    },
    build:{
        target:['edge90','chrome90','firefox90','safari15']
    },
    test: {
        include: ['src/**/*.{test,spec}.{js,ts}']
    }
});