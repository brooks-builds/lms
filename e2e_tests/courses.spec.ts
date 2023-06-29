import { expect, test } from "@playwright/test";
import { interceptGraphql } from "./graphql_intercepter";

test.describe("visitor", async () => {
  test("navigate to a live course", async ({ page }) => {
    await interceptGraphql(page);
    await page.goto("/courses", { waitUntil: "networkidle" });
    await page.getByText(/Cool course/).click();

    expect(page.url()).toMatch(/courses\/2/);
  });

  test("navigate to a featured course", async ({ page }) => {
    await interceptGraphql(page);
    await page.goto("/courses", { waitUntil: "networkidle" });
    await page.getByText(/Yew 0.20/).click();

    expect(page.url()).toMatch(/courses\/1/);
  });
});

