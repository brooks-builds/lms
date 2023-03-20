import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { courseListMockData } from "./mock_data";

const GRAPHQL_URI = process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";
const routes = [
	"/",
	"/courses",
	"/auth/create_account",
	"/auth/login",
	"/auth/redirect",
	"/tags",
];

for (let route of routes) {
	if (route != "/tags") continue;

	test.only(`${route} should not have any automatically detectable accessibility issues`, async ({ page }) => {
		await page.route(GRAPHQL_URI, async route => {
			const json = { "data": courseListMockData };
			await route.fulfill({ json });
		});
		await page.goto(route, { waitUntil: 'networkidle' });

		const accessibilityScanResults = await new AxeBuilder({ page }).analyze();

		expect(accessibilityScanResults.violations).toEqual([]);
	});
}
