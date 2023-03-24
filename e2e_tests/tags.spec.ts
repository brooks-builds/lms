import test, { expect } from "@playwright/test";
import { tagsMockData } from "./mock_data";
import { login, Role } from "./utils";
import { faker } from "@faker-js/faker";

const GRAPHQL_URI = process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

test("Author can navigate to the tags page", async ({ page }) => {
	await login(Role.Author, page);
	await page.getByRole("link", { name: "tags" }).first().click();

	await expect(page.url()).toMatch(/tags/);
});

// test("Learner cannot navigate to the tags page");

test("Author can see a list of existing tags", async ({ page }) => {
	await page.route(`${GRAPHQL_URI}`, route => route.fulfill({ json: tagsMockData() }));
	await login(Role.Author, page);
	await page.goto("/tags");

	for (let lmsTag of tagsMockData().data.lms_tags) {
		await expect(page.getByText(lmsTag.name)).toBeVisible();
	}
});

test.only("Author can create a new Tag", async ({ page }) => {
	const tagName = faker.random.word();

	await page.route(`${GRAPHQL_URI}`, route => {
		const request = route.request();
		const postData = request.postData() || "";
		const tags = tagsMockData();

		if (postData.search(/mutation/)) {
			tags.data.lms_tags.push({ id: 50, name: tagName });
		}

		return route.fulfill({ json: tags })
	});
	await login(Role.Author, page);
	await page.goto("/tags");

	await expect(page.getByText(tagName)).toBeVisible({ visible: false });

	const tagNameInput = page.getByLabel("Tag Name");
	await tagNameInput.type(tagName);
	await tagNameInput.press("Enter");
	const inputValue = await tagNameInput.inputValue();
	expect(inputValue).toBe("");

	await expect(page.getByText(tagName)).toBeVisible();
});

// test("Author can create a new Tag by pressing the submit button", async ({ page }) => {
// test("Author cannot create a tag with the same name");
// test("Author cannot create a tag with an empty name");
