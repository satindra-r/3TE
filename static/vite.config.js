import { defineConfig } from 'vite';

export default defineConfig({
	server: {
		fs: {
			allow: ['..']
		}
	},
	optimizeDeps: {
		exclude: ['../pkg/LearningWASM.js']
	},
	build: {
		target: 'esnext'
	},
	assetsInclude: ['**/*.wasm']
});