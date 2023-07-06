import { test, expect } from "@playwright/test";
import { interceptGraphql } from "./graphql_intercepter";

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