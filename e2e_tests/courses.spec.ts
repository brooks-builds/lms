import test, { expect } from "@playwright/test";
import { login, Role } from "./utils";

test("Author can create a course", async ({ page }) => {
	await login(Role.Author, page);
	await page.goto("/", { waitUntil: "networkidle" });
	await page.getByRole("link", { name: "Create Course" }).first().click();

	await expect(page.url()).toMatch(/create_course/);


});