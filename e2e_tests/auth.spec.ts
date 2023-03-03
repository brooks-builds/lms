import { expect, test } from "@playwright/test";
import { faker } from "@faker-js/faker";

const GRAPHQL_URI = process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

test("can create account", async ({ page }) => {
	await page.route(GRAPHQL_URI, route => {
		const body = route.request().postData();
		// validate that the body is correct
		// request should look something like
		// {"variables":{"email":"quen4whenairtn@aroisetnaorsietn.com","password":"arstarstarst"},"query":"query ListLmsCourses {\n  lms_courses {\n    id\n    lms_tag {\n      id\n      name\n    }\n    short_description\n    title\n\tprice\n\tlong_description\n\ttrailer_uri\n  }\n}\n\nquery CourseById($id: Int!) {\n  lms_courses_by_pk(id: $id) {\n    id\n    lms_tag {\n      id\n      name\n    }\n    price\n    short_description\n    title\n\tlong_description\n\ttrailer_uri\n  }\n}\n\nmutation CreateLmsAccount($email: String!, $password: String!) {\n  create_account(user: {email: $email, password: $password}) {\n    _id\n  }\n}\n","operationName":"CreateLmsAccount"}
	});
	await page.goto("/", { waitUntil: 'networkidle' });
	await page.getByRole('link', { name: "Get Started" }).click();
	await expect(page.url()).toMatch(/auth\/create/);
	await page.getByLabel('Email').fill(faker.internet.email(undefined, undefined, "mailinator.com"));
	await page.getByLabel('Password').fill(faker.internet.password());
	await page.getByRole('button').click();
});

