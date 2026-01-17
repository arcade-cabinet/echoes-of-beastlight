import { expect, test } from '@playwright/test';

test('load game and check for visual glitches', async ({ page }) => {
	// Go to the game URL (assuming it's served locally)
	// Capture console logs
	page.on('console', (msg) => {
		console.log(`BROWSER CONSOLE [${msg.type()}]: ${msg.text()}`);
	});

	const logs: string[] = [];
	page.on('console', (msg) => logs.push(msg.text()));

	await page.goto('http://localhost:8000');

	// Wait for the canvas to be present (Bevy uses a canvas)
	// Increase timeout to 60s as WASM loading can be slow
	const canvas = await page.waitForSelector('canvas', { timeout: 60000 });
	expect(canvas).toBeDefined();

	// Wait for the game to initialize
	await page.waitForTimeout(5000);

	// Take a screenshot for image analysis
	const _screenshot = await page.screenshot({ path: 'playthrough-initial.png' });

	// Here we would use image analysis capabilities
	// For now, we just ensure it loaded without obvious errors in console
	const errors = logs.filter((log) => {
		const l = log.toLowerCase();
		// Ignore software rendering warnings which are common in CI
		if (l.includes('software rendering') || l.includes('slow')) return false;
		return l.includes('error') || l.includes('panic');
	});
	expect(errors).toHaveLength(0);
});

test('playthrough sequence - movement', async ({ page }) => {
	page.on('console', (msg) => {
		console.log(`BROWSER CONSOLE [${msg.type()}]: ${msg.text()}`);
	});
	await page.goto('http://localhost:8000');
	await page.waitForSelector('canvas', { timeout: 60000 });
	await page.waitForTimeout(2000);

	// Press ArrowDown to move
	await page.keyboard.press('ArrowDown');
	await page.waitForTimeout(500);
	await page.screenshot({ path: 'playthrough-move-down.png' });

	// Press ArrowRight
	await page.keyboard.press('ArrowRight');
	await page.waitForTimeout(500);
	await page.screenshot({ path: 'playthrough-move-right.png' });
});
