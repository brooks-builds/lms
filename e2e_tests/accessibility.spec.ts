import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { courseListMockData, tagsMockData } from "./mock_data";
import { login, Role } from './utils';

const GRAPHQL_URI = process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";
const routes = [
	"/",
	"/courses",
	"/auth/create_account",
	"/auth/login",
	"/auth/redirect",
	"/create_article",
	// "/course_articles" // In order to support course articles we need to create a centralized graphql intercept with mock data
];

for (let route of routes) {
	test(`${route} should not have any automatically detectable accessibility issues`, async ({ page }) => {
		await page.route(GRAPHQL_URI, async route => {
			const json = { "data": courseListMockData };
			await route.fulfill({ json });
		});
		await login(Role.Author, page, route);

		const accessibilityScanResults = await new AxeBuilder({ page }).analyze();

		expect(accessibilityScanResults.violations).toEqual([]);
	});
}

test(`/tags should not have any automatically detectable accessibility issues`, async ({ page }) => {
	await page.route(GRAPHQL_URI, async route => {
		const json = tagsMockData();
		await route.fulfill({ json });
	});
	await login(Role.Author, page, "/tags");

	const accessibilityScanResults = await new AxeBuilder({ page }).analyze();

	expect(accessibilityScanResults.violations).toEqual([]);
});
