import { expect, test } from "@playwright/test";
import { interceptGraphql } from "./graphql_intercepter";

test.describe("visitor", async () => {
  test("can preview course", async ({ page }) => {
    await interceptGraphql(page);
    await page.goto("/courses/2", { waitUntil: "networkidle" });
    await page.getByRole("link", { name: /Preview/ }).click()
    expect(page.url()).toMatch(/courses\/2\/access/)
  });
});
