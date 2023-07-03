import { test, expect } from "@playwright/test";
import { interceptGraphql } from "./graphql_intercepter";
import { Role, login } from "./utils";

test.beforeEach(async ({ page }) => {
	await interceptGraphql(page);
});

test.describe("Visitor", async () => {
	test.beforeEach(async ({ page }) => {
		await login(Role.None, page, "/");
	});

	test("cannot see the tags link", async ({ page }) => {
		expect(await page.getByRole("link", { name: "Tags" }).isVisible()).toBe(false);
	});

	test("cannot load the tags page", async ({ page }) => {
		await page.goto("/tags", { waitUntil: "networkidle" });
		expect(page.url()).not.toMatch(/tags/);
		expect(await page.getByText("Authors can manage tags").isVisible()).toBe(true);
	})
});
