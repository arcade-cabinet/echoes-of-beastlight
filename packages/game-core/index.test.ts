import { describe, expect, it } from 'vitest';
import * as gameCore from './index';

describe('game-core', () => {
	it('should export VERSION', () => {
		expect(gameCore.VERSION).toBeDefined();
		expect(gameCore.VERSION).toBe('1.0.0');
	});
});
