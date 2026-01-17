import path from 'node:path';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [react()],
	server: {
		port: 3000,
		open: true,
	},
	resolve: {
		alias: {
			'@': path.resolve(__dirname, './src'),
			'@game-core': path.resolve(__dirname, '../game-core'),
		},
	},
});
