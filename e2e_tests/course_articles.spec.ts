import { expect, test } from "@playwright/test";
import { Role, login } from "./utils";
import { interceptGraphql } from "./graphql_intercepter";

test.beforeEach(async ({ page }) => {
  await interceptGraphql(page);
});

test.describe("Author", async () => {
  test.beforeEach(async ({ page }) => {
    await login(Role.Author, page, "/course_articles/1");
  })

  test("add an article to a course", async ({ page }) => {
    await page.getByTestId("articles")
      .filter({ has: page.getByRole("heading", { name: "All Articles" }) })
      .getByRole("button", { name: "Long Article" })
      .click();

    const assignedLongArticle = page.getByTestId("articles")
      .filter({ has: page.getByRole("heading", { name: "Assigned" }) })
      .getByRole("button", { name: "Long Article" });
    expect(await assignedLongArticle.isVisible()).toBe(true);

    expect(await page.getByText("Article added to course").isVisible()).toBe(true);

    await page.getByRole("button", { name: "Save" }).click();

    expect(await page.getByText("Course articles saved").isVisible()).toBe(true);
  })

  test("remove an article from a course", async ({ page }) => {
    await page.getByTestId("articles")
      .filter({ has: page.getByRole("heading", { name: "Assigned" }) })
      .getByRole("button", { name: "created in hasura" })
      .click();

    expect(await page.getByTestId("articles").filter({ has: page.getByRole("heading", { name: "All Articles" }) }).getByRole("button", { name: "created in hasura" }).isVisible()).toBe(true);

    expect(await page.getByText("Article removed from course").isVisible()).toBe(true);

    await page.getByRole("button", { name: "Save" }).click();

    expect(await page.getByText("saved").isVisible()).toBe(true);
  })
})

test.describe("Visitor", async () => {
  test.beforeEach(async ({ page }) => {
    await login(Role.None, page, "/courses/1");
  });

  test("cannot see the course articles link", async ({ page }) => {
    expect(await page.getByRole("link", { name: "Course Articles" }).isVisible()).toBe(false);
  });

  test("cannot load the course articles page", async ({ page }) => {
    await page.goto("/course_articles/1", { waitUntil: "networkidle" });
    expect(page.url()).not.toMatch(/course_articles/);
    expect(await page.getByText("Only Authors can manage course articles").isVisible()).toBe(true);
  })
});

test.describe("Learners", async () => {
  test.beforeEach(async ({ page }) => {
    await login(Role.Learner, page, "/courses/1");
  });

  test("cannot see the course articles link", async ({ page }) => {
    expect(await page.getByRole("link", { name: "Course Articles" }).isVisible()).toBe(false);
  });

  test("cannot load the course articles page", async ({ page }) => {
    await page.goto("/course_articles/1", { waitUntil: "networkidle" });
    expect(page.url()).not.toMatch(/course_articles/);
    expect(await page.getByText("Only Authors can manage course articles").isVisible()).toBe(true);
  })
});

