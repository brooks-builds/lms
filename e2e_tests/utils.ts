import { Page } from '@playwright/test'
import { learnerInfoMockData, userinfoMockData } from "./mock_data";
import * as dotenv from "dotenv";

dotenv.config();

const AUTH0_DOMAIN = process.env.AUTH0_DOMAIN;

export enum Role {
	Learner = 'Learner',
	Author = 'Author',
}

const userInfo = {
	Learner: learnerInfoMockData,
	Author: userinfoMockData,
}

export async function login(role: Role, page: Page): Promise<void> {
	await page.goto("/", { waitUntil: "networkidle" });
	await page.route(`${AUTH0_DOMAIN}/userinfo`, route => {
		
		return route.fulfill({ json: userInfo[role] })
	});
	await page.context().addCookies([{
		name: "auth_token",
		value: "1234qwfp1234qwfp",
		url: page.url(),
	}]);
	await page.goto("/", { waitUntil: "networkidle" });
}

