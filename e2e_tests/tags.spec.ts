import { test, expect } from "@playwright/test";
import { interceptGraphql } from "./graphql_intercepter";
import { Role, login } from "./utils";
import { faker } from "@faker-js/faker";

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

test.describe("Learners", async () => {
	test.beforeEach(async ({ page }) => {
		await login(Role.Learner, page, "/");
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

test.describe("Authors", async () => {
	test.beforeEach(async ({ page }) => {
		await login(Role.Author, page, "/tags");
	});

	test("can navigate to tags", async ({ page }) => {
		expect(await page.getByRole("link", { name: "Tags" }).first().isVisible()).toBe(true);
	});

	test("can create a tag", async ({ page }) => {
		const tagName = faker.random.word();

		const tagInput = await page.getByLabel("Tag Name")

		await tagInput.type(tagName, { delay: 50 });
		await page.getByRole("button", { name: "Create Tag" }).click();

		expect(await page.getByText("Tag Created").isVisible()).toBe(true);
		expect(await page.getByText(tagName).isVisible()).toBe(true);
		expect(await tagInput.inputValue()).toBe("");
	})
});
