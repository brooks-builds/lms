import { test, expect } from "@playwright/test";
import { faker } from "@faker-js/faker";

const GRAPHQL_URI =
  process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

test.beforeEach(async ({ page }) => {
  await page.goto("/auth/create_account", { waitUntil: "networkidle" });
});

test("can create an account", async ({ page }) => {
  const email = faker.internet.email(undefined, undefined, "mailinator.com");
  const password = faker.internet.password();
  let graphqlCalled = false;

  await page.route(GRAPHQL_URI, async (route, request) => {
    const body = request.postData();
    graphqlCalled = true;

    expect(body?.includes(email)).toBe(true);
    expect(body?.includes(password)).toBe(true);

    const responseBody = { data: { create_account: { email } } };
    route.fulfill({ body: JSON.stringify(responseBody) })
  })

  await page.getByLabel("email").type(email);
  await page.getByLabel("password").type(password);
  await page.getByRole("button", { name: "Create Account" }).click();

  await expect(page.getByText("Account created")).toBeVisible();
  expect(page.url()).toMatch(/login/);
  expect(graphqlCalled).toBe(true);
});

test("cannot create an account with missing email", async ({ page }) => {
  const password = faker.internet.password();
  let graphqlCalled = false;

  await page.route(GRAPHQL_URI, async (route, request) => {
    const body = request.postData();
    graphqlCalled = true;
  });

  await page.getByLabel("password").type(password);
  await page.getByRole("button", { name: "Create Account" }).click();

  await expect(page.getByText("must be an email and required")).toBeVisible();
  expect(page.url()).toMatch(/create_account/);
  expect(graphqlCalled).toBe(false);
});

test("cannot create an account with a string that is not an email", async ({ page }) => { });

test("cannot create an account with a password", async ({ page }) => { });

test("cannot create an account with a password that doesn't match requirements", async ({ page }) => { });
