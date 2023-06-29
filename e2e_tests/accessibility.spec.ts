import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { login, Role } from "./utils";
import { interceptGraphql } from "./graphql_intercepter";

const routes = [
  { path: "/", role: Role.None },
  { path: "/auth/create_account", role: Role.None },
  { path: "/auth/login", role: Role.None },
  { path: "/courses", role: Role.None },
  { path: "/courses/2", role: Role.None },
  { path: "/courses/2", role: Role.Learner },
  { path: "/courses/1", role: Role.Learner },
];

for (let route of routes) {
  test(`${route.path} as ${route.role} should not have any automatically detectable accessibility issues`, async ({
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
