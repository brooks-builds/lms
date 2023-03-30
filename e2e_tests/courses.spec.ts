import { faker } from "@faker-js/faker";
import test, { expect } from "@playwright/test";
import { login, Role } from "./utils";

test("Author can create a course", async ({ page }) => {
	await login(Role.Author, page);
	await page.goto("/", { waitUntil: "networkidle" });
	await page.getByRole("link", { name: "Create Course" }).first().click();

	expect(page.url()).toMatch(/create_course/);

	const longDescription = faker.lorem.paragraphs();
	await page.getByLabel("Long Description").type(longDescription);

	const price = faker.random.numeric();
	await page.getByLabel("Price (in dollars)").type(price);

	const shortDescription = faker.lorem.words(15);
	await page.getByLabel("Short Description").type(shortDescription);

	const possibleTags = ["Yew", "Axum"];
	const randomTag = possibleTags[Math.floor(Math.random() * 2)];
	await page.getByLabel("Tag").selectOption(randomTag);
});

test("Learner cannot create a course", async ({ page }) => {
	await login(Role.Learner, page);
	const createCourseLink = page.getByRole("link", { name: "Create Course" });

	await expect(createCourseLink).not.toBeVisible();

	await page.goto('/create_course', { waitUntil: "networkidle" });
	expect(page.url()).not.toMatch(/create_course/);
	await expect(page.getByText("Only Authors can create courses")).toBeVisible();
});

test("Not logged in users cannot create a course", async ({ page }) => {
	await page.goto("/")
	const createCourseLink = page.getByRole("link", { name: "Create Course" });

	await expect(createCourseLink).not.toBeVisible();

	await page.goto('/create_course', { waitUntil: "networkidle" });
	expect(page.url()).not.toMatch(/create_course/);
	await expect(page.getByText("Only Authors can create courses")).toBeVisible();
});
