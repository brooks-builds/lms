import { Page } from '@playwright/test'
import { userinfoMockData } from "./mock_data";

const AUTH0_DOMAIN = process.env.AUTH0_DOMAIN;

export enum Role {
	Learner = 'Learner',
	Author = 'Author',
}

export async function login(role: Role, page: Page): Promise<void> {
	await page.goto("/", { waitUntil: "networkidle" });
	await page.route(`${AUTH0_DOMAIN}/userinfo`, route => {
		return route.fulfill({ json: userinfoMockData })
	});
	await page.context().addCookies([{
		name: "auth_token",
		value: "1234qwfp1234qwfp",
		url: await page.url(),
	}]);
	await page.goto("/", { waitUntil: "networkidle" });
}
