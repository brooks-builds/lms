import { expect, test } from "@playwright/test";
import { faker } from "@faker-js/faker";
import { createAccount } from "./intercept_data";

const GRAPHQL_URI = process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

test("can create account", async ({ page }) => {
	const email = faker.internet.email(undefined, undefined, "mailinator.com");
	const password = faker.internet.password();
	let graphQlRoutesCalled = 0;

	await page.goto("/", { waitUntil: 'networkidle' });
	await page.getByRole('link', { name: "Get Started" }).click();
	await expect(page.url()).toMatch(/auth\/create/);

	await page.route(GRAPHQL_URI, route => {
		const body = route.request().postData() || "";
		const expectedBody = createAccount(email, password);

		expect(JSON.parse(body)).toEqual(expectedBody);
		graphQlRoutesCalled += 1;
	});

	await page.getByLabel('Email').fill(email);
	await page.getByLabel('Password').fill(password);
	await page.getByRole('button').click();

	expect(graphQlRoutesCalled).toBe(1);
});

