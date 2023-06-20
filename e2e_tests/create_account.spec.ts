import { test, expect } from "@playwright/test";
import { faker } from "@faker-js/faker";

const GRAPHQL_URI =
  process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

test("can create an account", async ({ page }) => {
  await page.goto("/auth/create_account", { waitUntil: "networkidle" });

  const email = faker.internet.email(undefined, undefined, "mailinator.com");
  const password = faker.internet.password();
  let graphqlCalled = false;

  await page.route(GRAPHQL_URI, async (route, request) => {
    const body = request.postData();
    graphqlCalled = true;

    expect(body?.includes(email)).toBe(true);
    expect(body?.includes(password)).toBe(true);
  })

  await page.getByLabel("email").type(email);
  await page.getByLabel("password").type(password);
  await page.getByRole("button", { name: "Create Account" }).click();

  expect(graphqlCalled).toBe(true);
})
