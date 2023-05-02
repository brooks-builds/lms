import { expect, test } from "@playwright/test";
import { interceptGraphql } from "./graphql_intercepter";

test("visitors should be able to preview a course", async ({ page }) => {
  await interceptGraphql(page);
  await page.goto("/");
  await page.getByRole("link", { name: "Courses" }).first().click();
  await page.waitForLoadState('networkidle');
  await page.getByRole("link", { name: "Yew.rs" }).click();
  await page.waitForLoadState('networkidle');
  await page.getByRole("link", { name: "Preview" }).click();
  expect(page.url()).toMatch(/access/);
  await page.waitForLoadState('networkidle');
  await expect(
    page.getByRole("heading", { name: "Introduction to Yew" })
  ).toBeVisible();
  const previewLinks = page.getByRole("link", { name: "preview" });
  await expect(previewLinks).toBeVisible();
  await previewLinks.first().click();
  await expect(page.url()).toMatch(/access\/2/);
  await expect(page.getByRole('heading', { name: 'This is my course' })).toBeVisible();
  await expect(page.getByText('Hello world')).toBeVisible();
});
