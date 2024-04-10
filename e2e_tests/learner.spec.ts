import { test } from "@playwright/test";
import dotenv from "dotenv";

dotenv.config({});

const TEST_LEARNER_USERNAME = process.env.TEST_LEARNER_USERNAME || "";
const TEST_LEARNER_PASSWORD = process.env.TEST_LEARNER_PASSWORD || "";

test("navigation", async ({ page }) => {
  await page
    .locator("#navbarSupportedContent")
    .getByRole("link", { name: "Courses" })
    .click();
  await page.getByRole("heading", { name: "Course Library" }).click();
  await page.getByRole("link", { name: "Brooks Builds Brand icon" }).click();
  await page.getByRole("heading", { name: "Brooks Builds Learning" }).click();
  await page
    .locator("#navbarSupportedContent")
    .getByRole("link", { name: "Courses" })
    .click();
  await page
    .locator("#navbarSupportedContent")
    .getByRole("link", { name: "Home" })
    .click();
  await page.getByRole("heading", { name: "Brooks Builds Learning" }).click();

  return;
});

test("login", async ({ page }): Promise<any> => {
  await page.goto("http://localhost:8082/");
  await page.getByRole("link", { name: "Login or Signup" }).click();
  await page.getByLabel("Email address").click();
  await page.getByLabel("Email address").fill(TEST_LEARNER_USERNAME);
  await page.getByLabel("Email address").press("Tab");
  await page.getByLabel("Password").fill(TEST_LEARNER_PASSWORD);
  await page.getByRole("button", { name: "Continue", exact: true }).click();
  await page.getByRole("heading", { name: "Course Library" }).click();
});

test("Visitors can preview a course", async ({ page }): Promise<any> => {
  await page.goto("http://localhost:8082/");
  await page
    .locator("#navbarSupportedContent")
    .getByRole("link", { name: "Courses" })
    .click();
  await page.getByRole("heading", { name: "Course Library" }).click();
  await page.getByRole("heading", { name: "Featured Courses" }).click();
  await page
    .getByTestId("courses")
    .getByRole("link", { name: /Docker/ })
    .click();
  await page.getByRole("link", { name: "Preview" }).click();
  await page
    .getByRole("button", { name: "Preview the article" })
    .first()
    .click();
  await page.getByRole("link", { name: "Complete and goto next" }).click();
  await page.getByRole("link", { name: "hello - (preview)" }).click();
  await page.getByRole("button", { name: "Buy this course" }).click();
});
