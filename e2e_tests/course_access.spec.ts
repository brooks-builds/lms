import { expect, test } from "@playwright/test";
import { interceptGraphql } from "./graphql_intercepter";

test.beforeEach(async ({ page }) => {
  await interceptGraphql(page);
});

test.describe("visitor", async () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/courses/2/access", { waitUntil: "networkidle" })
  })

  test("can see a preview article", async ({ page }) => {
    await page.getByRole("link", { name: "Cool article 5" }).click()
    await expect(page.getByText(/Yay I am an article/)).toBeVisible()
  });

  test.describe("log in to purchase button", async () => {
    test("visible when logged out", async ({ page }) => {
      await page.goto("/courses/2/access/1", { waitUntil: "networkidle" });
      const loginButton = await page.getByRole("button", { name: "Log in to purchase" });

      expect(loginButton).toBeVisible();

      await loginButton.click();

      expect(page.url()).toMatch(/login/);
    });

  });


  test.describe("complete article button", async () => {
    test("is disabled when viewing as a visitor", async ({ page }) => {
      await page.goto("/courses/2/access/1", { waitUntil: "networkidle" });
      const completeArticleButton = await page.getByRole("button", { name: "Complete Article" });
      expect(await completeArticleButton.isDisabled()).toBe(true);
    });

  })
});
