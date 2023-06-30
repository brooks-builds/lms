import { expect, test } from "@playwright/test";
import { interceptGraphql } from "./graphql_intercepter";
import { Role, login } from "./utils";

test.beforeEach(async ({ page }) => {
  await interceptGraphql(page);
});

test.describe("visitor", async () => {
  test("can preview course", async ({ page }) => {
    await page.goto("/courses/2", { waitUntil: "networkidle" });
    await page.getByRole("link", { name: /Preview/ }).click()
    expect(page.url()).toMatch(/courses\/2\/access/)
  });
});

test.describe("learner", async () => {
  test.beforeEach(async ({ page }) => {
    await login(Role.Learner, page, "/courses/1");
  });

  test("can purchase a course", async ({ page }) => {
    const purchaseButton = page.getByRole("link", { name: "purchase" });

    expect(await purchaseButton.isVisible()).toBe(true);
  });

  test("can preview course", async ({ page }) => {
    await page.getByRole("link", { name: /Preview/ }).click()
    expect(page.url()).toMatch(/courses\/1\/access/)
  });

  test("can open owned course", async ({ page }) => {
    await page.goto("/courses/2", { waitUntil: "networkidle" });
    await page.getByRole("link", { name: "Open" }).click();
    expect(page.url()).toMatch(/courses\/2\/access/);
  })
});

test.describe("author", async () => {
  test.beforeEach(async ({ page }) => {
    await login(Role.Author, page);
  });

  test("can navigate to the course articles page", async ({ page }) => {
    await page.goto("/courses/1", { waitUntil: "networkidle" });
    await page.getByRole("link", { name: "Course Articles" }).click();
    const url = page.url();

    expect(url).toMatch(/course_articles\/1/);
  })
})
