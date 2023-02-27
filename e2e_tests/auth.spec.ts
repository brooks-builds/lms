import { expect, test } from "@playwright/test";
import { faker } from "@faker-js/faker";

test("can create account", async ({ page }) => {
	await page.goto("/", {waitUntil: 'networkidle'});

	await page.getByRole('link', {name: "Get Started"}).click();
	await expect(page.url()).toMatch(/auth\/create/);
	await page.getByLabel('Email').fill(faker.internet.email(undefined, undefined, "mailinator.com"));
	await page.getByLabel('Password').fill(faker.internet.password());
	await page.getByRole('button').click();
});

