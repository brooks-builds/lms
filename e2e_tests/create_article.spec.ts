import { test, expect } from "@playwright/test";
import { GRAPHQL_URI, interceptGraphql } from "./graphql_intercepter";
import { Role, login } from "./utils";
import { faker } from "@faker-js/faker";
import apiCreateArticleJson from "./graphql_intercepter/api_insert_article.json";

test.beforeEach(async ({ page }) => { await interceptGraphql(page) });

test.describe("Visitor", async () => {
  test("cannot see the create article link", async ({ page }) => {
    await page.goto("/", { waitUntil: "domcontentloaded" });

    await page.waitForTimeout(50);

    expect(await page.getByRole("link", { name: "Create Article" }).isVisible()).toBe(false);
  });

  test("cannot navigate directly to create article page", async ({ page }) => {
    await page.goto("/create_article");
    await page.waitForURL("/");
    await page.waitForTimeout(150);

    const errorMessage = page.getByText("Authors can create");
    const isVisible = await errorMessage.isVisible({ timeout: 60000 });

    expect(isVisible).toBe(true);
  });
});

test.describe("Learners", async () => {
  test("cannot see the create article link", async ({ page }) => {
    await login(Role.Learner, page, "/create_article");
    await page.waitForURL("/");
    await page.waitForTimeout(50);

    expect(await page.getByRole("link", { name: "Create Article" }).isVisible()).toBe(false);
  });

  test("cannot navigate directly to create article page", async ({ page }) => {
    await login(Role.Learner, page, "/create_article");
    await page.waitForURL("/");
    await page.waitForTimeout(150);

    const errorMessage = page.getByText("Authors can create");
    const isVisible = await errorMessage.isVisible({ timeout: 60000 });

    expect(isVisible).toBe(true);
  });
});

test.describe("Authors", async () => {
  test.beforeEach(async ({ page }) => {
    await login(Role.Author, page, "/create_article");
    await page.waitForTimeout(50);
  });

  test("can navigate to the create article page", async ({ page }) => {
    await page.goto("/");
    await page.getByRole("link", { name: "Create Article" }).first().click();
    await page.waitForURL("/create_article");
    await page.waitForTimeout(50);

    expect(page.url()).toMatch(/create_article/);
  });

  test("can create an article", async ({ page }) => {
    const newTitle = faker.commerce.productName();
    const newContent = faker.lorem.paragraphs(3);
    let intercepted = false;

    await page.route(GRAPHQL_URI, async (route, request) => {
      const { title, content } = request.postDataJSON().variables;

      expect(title).toBe(newTitle);
      expect(content).toBe(newContent);

      intercepted = true;

      await route.fulfill({ json: { data: apiCreateArticleJson } });
    });

    await page.getByLabel("Title").type(newTitle);
    await page.getByLabel("Article Body").type(newContent);
    await page.getByRole("button", { name: "Create Article" }).click();

    expect(intercepted).toBe(true);
    expect(await page.getByText("created").isVisible()).toBe(true);
  });

  test.skip("cannot create an article without title", async ({ page }) => { });
  test.skip("cannot create an article without a body", async ({ page }) => { });
});