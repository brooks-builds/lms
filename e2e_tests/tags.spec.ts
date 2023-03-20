import test, { expect } from "@playwright/test";
import { tagsMockData } from "./mock_data";
import { login, Role } from "./utils";

const GRAPHQL_URI = process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

test("Author can navigate to the tags page", async ({ page }) => {
	await login(Role.Author, page);
	await page.getByRole("link", { name: "tags" }).first().click();

	await expect(page.url()).toMatch(/tags/);
});

test.only("Author can see a list of existing tags", async ({ page }) => {
	await page.route(`${GRAPHQL_URI}`, route => route.fulfill({ json: tagsMockData }));
	await login(Role.Author, page);
	await page.goto("/tags");

	for (let lmsTag of tagsMockData.data.lms_tags) {
		await expect(page.getByText(lmsTag.name)).toBeVisible();
	}
})
