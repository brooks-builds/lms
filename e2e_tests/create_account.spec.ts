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

  await page.waitForTimeout(25);

  await page.getByLabel("email").type(email);
  await page.getByLabel("password (required)").type(password, { delay: 10 });
  await page.getByRole("button", { name: "Create Account" }).click();

  await expect(page.getByText("Account created")).toBeVisible();
  expect(page.url()).toMatch(/login/);
  expect(graphqlCalled).toBe(true);
});

test("cannot create an account with missing email", async ({ page }) => {
  const password = faker.internet.password();
  let graphqlCalled = false;

  await page.waitForTimeout(100);

  await page.route(GRAPHQL_URI, async (_route, _request) => {
    graphqlCalled = true;
  });

  await page.getByLabel("password (required)").type(password);
  const button = page.getByRole("button", { name: "Create Account" });
  expect(await button.isDisabled()).toBe(true);
  expect(page.url()).toMatch(/create_account/);
  expect(graphqlCalled).toBe(false);
});

test("cannot create an account with a string that is not an email", async ({ page }) => {
  const email = faker.random.word();
  const password = faker.internet.password();

  await page.getByLabel("password (required)").type(password);
  await page.getByLabel("email").type(email);

  expect(await page.getByRole("button", { name: "Create Account" }).isDisabled()).toBe(true);
  await expect(page.getByText("must be an email")).toBeVisible();
});

test("cannot create an account without a password", async ({ page }) => {
  let email = faker.internet.email(undefined, undefined, "mailinator.com");
  await page.getByLabel("email").type(email);
  await page.waitForTimeout(500);

  expect(await page.getByRole("button", { name: "Create Account" }).isDisabled()).toBe(true);
});

test("cannot create an account with a password that doesn't match requirements", async ({ page }) => {
  let password = faker.random.alphaNumeric(7);
  let email = faker.internet.email(undefined, undefined, "mailinator.com");

  await page.waitForTimeout(500);
  await page.getByLabel("email").type(email);
  await page.getByLabel("password (required)").type(password);

  expect(await page.getByRole("button", { name: "Create Account" }).isDisabled()).toBe(true);
  await expect(page.getByText("required", { exact: true })).toBeVisible();
});

