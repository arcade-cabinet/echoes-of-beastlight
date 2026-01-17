import { describe, expect, it } from 'vitest';
import * as gameCore from './index';

describe('game-core', () => {
	it('should export modules', () => {
		expect(gameCore).toBeDefined();
	});
});
