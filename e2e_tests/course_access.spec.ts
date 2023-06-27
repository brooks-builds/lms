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

  test("can create an account", async ({ page }) => { throw new Error("stub") });

  test("can mark preview article completed", async ({ page }) => { throw new Error("stub") });
});
