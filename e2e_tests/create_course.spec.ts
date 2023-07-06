import { test, expect } from "@playwright/test";
import { faker } from "@faker-js/faker";
import { interceptGraphql } from "./graphql_intercepter";
import { Role, login } from "./utils";

const GRAPHQL_URI =
	process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

test.describe("Visitor", async () => {
	test("cannot see the create course link", async ({ page }) => {
		await interceptGraphql(page);
		await page.goto("/", { waitUntil: "networkidle" });
		expect(await page.getByRole("link", { name: "Create Course" }).isVisible()).toBe(false);
	});

	test("cannot navigate to the create course page", async ({ page }) => {
		await interceptGraphql(page);
		await page.goto("/create_course", { waitUntil: "networkidle" });
		await page.waitForURL("/");
		expect(await page.getByText("authors can create").isVisible()).toBe(true);
	});
});

test.describe("Learner", async () => {
	test("cannot see the create course link", async ({ page }) => {
		await interceptGraphql(page);
		await login(Role.Learner, page, "/");
		expect(await page.getByRole("link", { name: "Create Course" }).isVisible()).toBe(false);
	});

	test("cannot navigate to the create course page", async ({ page }) => {
		await interceptGraphql(page);
		await login(Role.Learner, page, "/create_course");
		await page.waitForURL("/");
		expect(await page.getByText("authors can create").isVisible()).toBe(true);
	});
});

test.describe("Author", async () => {
	test("can navigate to create course page", async ({ page }) => {
		await interceptGraphql(page);
		await login(Role.Author, page, "/");
		await page.getByRole("link", { name: "Create Course" }).first().click();
		await page.waitForTimeout(100);
		expect(page.url()).toMatch(/create_course/);
	});

	test("can create a course", async ({ page }) => {
		await interceptGraphql(page);

		const newCourseTitle = faker.company.bsBuzz();
		const tag = "Rust";
		const longDescription = faker.lorem.paragraphs(3);
		const shortDescription = faker.company.catchPhrase();
		const liveCourse = Math.random() > 0.49;

		await login(Role.Author, page, "/create_course");

		await page.getByLabel("Title").type(newCourseTitle);
		await page.getByLabel("Tag").selectOption(tag);
		await page.getByLabel("Long Description").type(longDescription);
		await page.getByLabel("Short Description").type(shortDescription);

		if (liveCourse) {
			await page.getByLabel("Live Course").check();
		}

		let intercepted = false;

		page.route(GRAPHQL_URI, async (_route, request) => {
			const body = request.postDataJSON();
			const { title, tag_id, long_description, short_description, live } = body.variables;

			expect(title).toBe(newCourseTitle);
			expect(tag_id).toBe(5);
			expect(long_description).toBe(longDescription);
			expect(short_description).toBe(shortDescription);
			expect(live).toBe(liveCourse);

			intercepted = true;
		});

		await page.getByRole("button", { name: "Create Course" }).click();

		expect(intercepted).toBe(true);
	});

	test("cannot create a course if info is missing", async ({ page }) => {
		await interceptGraphql(page);
		await login(Role.Author, page, "/create_course");

		expect(await page.getByRole("button").isDisabled()).toBe(true);
	});
});
