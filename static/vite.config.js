import { defineConfig } from 'vite';

export default defineConfig({
	base: '/3TE/',
	server: {
		fs: {
			allow: ['..']
		}
	},
	optimizeDeps: {
		exclude: ['../pkg/LearningWASM.js']
	},
	build: {
		target: 'esnext',
		outDir: '../dist',
		emptyOutDir: true
	},
	assetsInclude: ['**/*.wasm']
});