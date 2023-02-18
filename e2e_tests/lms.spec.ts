import { expect, test } from "@playwright/test";
import { courseListMockData } from "./mock_data";

const GRAPHQL_URI = process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

test.beforeEach(async ({ page }) => {
	await page.goto("/");
});

test("has title", async ({ page }) => {
	await expect(page).toHaveTitle(/Brooks Builds/)
});

test("can join discord", async ({ page }) => {
	const discordLink = await page.getByRole("link", { name: "Join Discord" });
	await expect(discordLink).toHaveAttribute("href", /discord.gg/)
});

test("can view a course", async ({ page }) => {
	await page.route(GRAPHQL_URI, async route => {
		const json = { "data": courseListMockData };
		await route.fulfill({ json });
	});

	await page.getByRole("link", { name: "Courses" }).first().click();
	await expect(page).toHaveURL(/\/courses/);
	await page.getByText(courseListMockData.lms_courses[0].short_description).click();
	await expect(page).toHaveURL(/\/courses\/1/);
});
