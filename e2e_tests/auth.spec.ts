import { expect, test } from "@playwright/test";
import { faker } from "@faker-js/faker";
import { createAccount } from "./intercept_data";
import { createAccountMockData, userinfoMockData } from "./mock_data";
import dotenv from "dotenv";

dotenv.config({});

const GRAPHQL_URI = process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";
const LOGIN_URI = process.env.LOGIN_URI || "";
const AUTH0_DOMAIN = process.env.AUTH0_DOMAIN;

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
		return route.fulfill({
			json: createAccountMockData,
		});
	});

	await page.getByLabel('Email').fill(email);
	await page.getByLabel('Password').fill(password);
	await page.getByRole('button').click();

	expect(graphQlRoutesCalled).toBe(1);

	await page.waitForURL('/auth/login');

	await expect(await page.getByText(/Created/)).toBeVisible()
	await expect(await page.url()).toMatch(/login/)
});

test("can login to an account", async ({ page }) => {
	if (!LOGIN_URI.length) throw new Error("missing LOGIN URI");

	await page.goto("/auth/login", { waitUntil: 'networkidle' });
	await expect(page.url()).toMatch(/login/);
	const loginLink = await page.getByRole('link', { name: 'username' });
	await expect(await loginLink.getAttribute('href')).toMatch(LOGIN_URI);
});

test.only("auth redirect should work", async ({ page, context }) => {
	await page.route(`${AUTH0_DOMAIN}/userinfo`, route => route.fulfill({ json: userinfoMockData }));
	await page.goto("/auth/login", { waitUntil: "networkidle" });

	const cookies = await page.context().cookies();
	const stateCookie = cookies.find(cookie => {
		console.log(cookie, cookie.name == "auth_state");
		return cookie.name == "auth_state"
	});

	if (!stateCookie) throw new Error("state cookie doesn't exist");

	const redirectUri = `/auth/redirect#access_token=1234qwfpdrfyupg&scope=openid%20profile%20email&expires_in=7200&token_type=Bearer&state=${stateCookie.value}`;

	await page.goto(redirectUri, { waitUntil: "networkidle" });

	await expect(await page.getByText(/Welcome, meow/)).toBeVisible();
});

test("revisiting website after logging in should work", async ({ page, context }) => {
	await page.route(`${AUTH0_DOMAIN}/userinfo`, route => route.fulfill({ json: userinfoMockData }));
	await page.goto("/", { waitUntil: "networkidle" });
	await page.context().addCookies([{
		name: "auth_token",
		value: "1234qwfp1234qwfp",
		url: await page.url(),
	}]);
	await page.goto("/", { waitUntil: "networkidle" });
	await expect(await page.getByText(/Welcome, meow/)).toBeVisible();
});
