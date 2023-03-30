import test, { expect } from "@playwright/test";
import { login, Role } from "./utils";

test("Author can create a course", async ({ page }) => {
	await login(Role.Author, page);
	await page.goto("/", { waitUntil: "networkidle" });
	await page.getByRole("link", { name: "Create Course" }).first().click();

	expect(page.url()).toMatch(/create_course/);
});

test("Learner cannot create a course", async ({ page }) => {
	await login(Role.Learner, page);
	const createCourseLink = page.getByRole("link", { name: "Create Course" });

	await expect(createCourseLink).not.toBeVisible();
});

test("Not logged in users cannot create a course", async ({ page }) => {
	await page.goto("/")
	const createCourseLink = page.getByRole("link", { name: "Create Course" });

	await expect(createCourseLink).not.toBeVisible();
});
