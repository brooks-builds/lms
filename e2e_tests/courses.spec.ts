import { faker } from "@faker-js/faker";
import test, { expect } from "@playwright/test";
import { login, Role } from "./utils";
import { courseListMockData, lmsArticlesMockData, lmsCoursByPk, tagsMockData } from "./mock_data";

const GRAPHQL_URI = process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

test("Author can create a course", async ({ page }) => {
	await page.route(GRAPHQL_URI, async route => {
		const json = tagsMockData();
		return route.fulfill({json});
	})
	await login(Role.Author, page);
	await page.goto("/", { waitUntil: "networkidle" });
	await page.getByRole("link", { name: "Create Course" }).first().click();

	expect(page.url()).toMatch(/create_course/);

	const longDescription = faker.lorem.paragraphs();
	await page.getByLabel("Long Description").type(longDescription);

	const shortDescription = faker.lorem.words(15);
	await page.getByLabel("Short Description").type(shortDescription);

	await page.getByTestId("select").selectOption("Yew");

	const title = faker.random.words(3);
	await page.getByLabel("Title").type(title);

	await page.route(GRAPHQL_URI, async route => {
		let json = lmsCoursByPk(100, title, shortDescription, longDescription, "Yew");
		route.fulfill({json});
	});

	await page.getByRole('button', {name: 'Create Course'}).click();
	await page.waitForURL(/courses/, {waitUntil: "domcontentloaded"});
	expect(page.url()).toMatch(/courses\/100/);
});

test("Learner cannot create a course", async ({ page }) => {
	await login(Role.Learner, page);
	const createCourseLink = page.getByRole("link", { name: "Create Course" });

	await expect(createCourseLink).not.toBeVisible();

	await page.goto('/create_course', { waitUntil: "networkidle" });
	expect(page.url()).not.toMatch(/create_course/);
	await expect(page.getByText("Only Authors can create courses")).toBeVisible();
});

test("Not logged in users cannot create a course", async ({ page }) => {
	await page.goto("/")
	const createCourseLink = page.getByRole("link", { name: "Create Course" });

	await expect(createCourseLink).not.toBeVisible();

	await page.goto('/create_course', { waitUntil: "networkidle" });
	expect(page.url()).not.toMatch(/create_course/);
	await expect(page.getByText("Only Authors can create courses")).toBeVisible();
});

test("Author add articles to a course", async ({page}) => {
	await login(Role.Author, page);
	await page.route(GRAPHQL_URI, async route => {
		return route.fulfill({json: {data: courseListMockData}});
	})

	await page.goto("/courses", {waitUntil: "networkidle"});
	await page.getByText("Yew.rs").first().click();

	const articles = [
		faker.commerce.productName(),
	];

	await page.route(GRAPHQL_URI, async route => {
		 return route.fulfill({json: lmsArticlesMockData(articles)});
	});

	await page.getByRole("link", {name: "Course Articles"}).click();
	await expect(page.getByText(articles[0])).toBeVisible();
});
