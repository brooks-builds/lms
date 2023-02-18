import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test.describe('homepage', () => {
	test('homepage should not have any automatically detectable accessibility issues', async ({ page }) => {
		await page.goto('/', { waitUntil: 'networkidle' });

		const accessibilityScanResults = await new AxeBuilder({ page }).analyze();

		expect(accessibilityScanResults.violations).toEqual([]);
	});
});

test.describe('courses', () => {
	test('courses should not have any automatically detectable accessibility issues', async ({ page }) => {
		await page.goto('/');

		const accessibilityScanResults = await new AxeBuilder({ page }).analyze();

		expect(accessibilityScanResults.violations).toEqual([]);
	});
});
