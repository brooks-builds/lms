import test, { expect } from "@playwright/test";
import { createdTagMockData, tagsMockData } from "./mock_data";
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

test("Author can create a new Tag", async ({ page }) => {
	const tagName = faker.random.word();

	await page.route(`${GRAPHQL_URI}`, route => {
		const request = route.request();
		const postData = request.postData() || '';
		const tags = tagsMockData();

		return route.fulfill({ json: tags })
	});
	await login(Role.Author, page);
	await page.getByRole("link", { name: "Tags" }).first().click();

	await expect(page.getByText(tagName)).toBeVisible({ visible: false });

	await page.route(`${GRAPHQL_URI}`, route => {
		const request = route.request();
		const postData = request.postData() || '';
		const tags = createdTagMockData(tagName);

		return route.fulfill({ json: tags })
	});

	const tagNameInput = page.getByLabel("Tag Name");
	await tagNameInput.type(tagName);
	await tagNameInput.press("Enter");
	const inputValue = await tagNameInput.inputValue();
	expect(inputValue).toBe("");

	await expect(page.getByText(tagName)).toBeVisible();
});

test("Author can create a new Tag by pressing the submit button", async ({ page }) => {
	const tagName = faker.random.word();

	await page.route(`${GRAPHQL_URI}`, route => {
		const tags = tagsMockData();

		return route.fulfill({ json: tags })
	});
	await login(Role.Author, page);
	await page.getByRole("link", { name: "Tags" }).first().click();

	await expect(page.getByText(tagName)).toBeVisible({ visible: false });

	await page.route(`${GRAPHQL_URI}`, route => {
		const tags = createdTagMockData(tagName);

		return route.fulfill({ json: tags })
	});

	const tagNameInput = page.getByLabel("Tag Name");
	await tagNameInput.type(tagName);
	await page.getByRole("button", {name: "Create Tag"}).click();
	const inputValue = await tagNameInput.inputValue();
	expect(inputValue).toBe("");

	await expect(page.getByText(tagName)).toBeVisible();
});

test("Author can't create a new Tag with an empty name", async ({ page }) => {
	await page.route(`${GRAPHQL_URI}`, route => {
		const tags = tagsMockData();

		return route.fulfill({ json: tags })
	});
	await login(Role.Author, page);
	await page.getByRole("link", { name: "Tags" }).first().click();
	await page.getByRole("button", {name: "Create Tag"}).click();
	await expect(page.getByText("Cannot create a tag without a name")).toBeVisible();
});

test("learner can't navigate to the tags page", async ({page}) => {
	await login(Role.Learner, page);
	await expect(page.getByText("Author")).not.toBeVisible();
	await expect(page.getByRole("link", {name: "Tags"})).not.toBeVisible();
	await page.goto("/tags", {waitUntil: "networkidle"});
	await expect(page.getByText("Only Authors can manage tags")).toBeVisible();
})

