import wasmPack from 'vite-plugin-wasm-pack';
import {sveltekit} from '@sveltejs/kit/vite';
import {defineConfig} from 'vite';
import {SvelteKitPWA} from "@vite-pwa/sveltekit"
export default defineConfig({
    plugins: [
        sveltekit(), wasmPack('./src/lib/wasm-lib'), SvelteKitPWA(
            {
                strategies: 'injectManifest', srcDir: 'src', filename: 'my-sw.js',
                // or `my-sw.ts`
                /* other pwa options */
            }
        )
    ]


});
