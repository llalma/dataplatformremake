import { sveltekit } from '@sveltejs/kit/vite';
import wasmPack from 'vite-plugin-wasm-pack';
import { defineConfig } from 'vite'

export default defineConfig({
	server: {
		fs: {
			// Allow serving files from one level up to the project root
			allow: ['..']
		}
	},
	plugins: [sveltekit(),
				wasmPack('./rustFunctions/grid')]
})
