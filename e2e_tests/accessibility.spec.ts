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
  { path: "/courses", role: Role.None },
  { path: "/courses/1", role: Role.None },
  { path: "/create_course", role: Role.Author },
  { path: "/tags", role: Role.Author },
  { path: "/create_article", role: Role.Author },
  { path: "/course_articles/1", role: Role.Learner },
  { path: "/courses/1/access", role: Role.Learner },
  { path: "/courses/1/access/1", role: Role.Learner },
  { path: "/courses/1/purchase", role: Role.Learner },
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
