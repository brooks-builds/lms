import { test, expect } from "@playwright/test";
import { interceptGraphql } from "./graphql_intercepter";
import { Role, login } from "./utils";

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
  test("can navigate to the create article page", async ({ page }) => {
    await login(Role.Author, page, "/");
    await page.waitForTimeout(50);
    await page.getByRole("link", { name: "Create Article" }).first().click();
    await page.waitForURL("/create_article");
    await page.waitForTimeout(50);

    expect(page.url()).toMatch(/create_article/);
  });
});