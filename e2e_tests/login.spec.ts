import { expect, test } from "@playwright/test";
import { interceptGraphql } from "./graphql_intercepter";

test("login page links to auth0", async ({ page }) => {
  await interceptGraphql(page);
  await page.goto("/auth/login", { waitUntil: "networkidle" });

  const authLink = page.getByRole('link', { name: "Username and Password" });

  expect(await authLink.getAttribute('href')).toMatch(/auth0.com/);
});