import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { tagsMockData } from "./mock_data";
import { login, Role } from "./utils";
import { interceptGraphql } from "./graphql_intercepter";

const GRAPHQL_URI =
  process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";
const routes = [
  { path: "/", role: Role.None },
  { path: "/auth/create_account", role: Role.None },
  { path: "/auth/login", role: Role.None },
];

for (let route of routes) {
  test(`${route.path} should not have any automatically detectable accessibility issues`, async ({
    page,
  }) => {
    await interceptGraphql(page);
    if (route.role == Role.None) {
      await page.goto(route.path, { waitUntil: "networkidle" });
    } else {
      await login(route.role, page, route.path);
    }

    const accessibilityScanResults = await new AxeBuilder({ page }).analyze();

    expect(accessibilityScanResults.violations).toEqual([]);
  });
}
