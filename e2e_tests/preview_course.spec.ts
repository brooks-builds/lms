import { expect, test } from "@playwright/test";
import { interceptGraphql } from "./graphql_intercepter";

test("visitors should be able to preview a course", async ({ page }) => {
  await interceptGraphql(page);
  await page.goto("/");
  await page.getByRole("link", { name: "Courses" }).first().click();
  await page.getByRole("link", { name: "Yew.rs" }).click();
  await page.getByRole("link", { name: "Preview" }).click();
  expect(page.url()).toMatch(/access/);
  await expect(
    page.getByRole("heading", { name: "Introduction to Yew" })
  ).toBeVisible();
  await expect(page.getByRole("link", { name: "How to Learn" })).toBeVisible();
});
