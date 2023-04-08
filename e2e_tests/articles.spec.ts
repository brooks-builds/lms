import {test, expect} from "@playwright/test";
import {faker} from "@faker-js/faker";
import {Role, login} from "./utils";
import { createArticleIntercept } from "./intercept_data";
import { createArticleMockData } from "./mock_data";

const GRAPHQL_URI = process.env.GRAPHQL_URI || "http://localhost:8081/v1/graphql";

test("An author can create an article", async ({page}) => {
  await login(Role.Author, page);
  
  await page.getByRole("link", {name: "Create Article"}).first().click();
  expect(page.url()).toMatch(/create_article/);

  const randomTitle = faker.lorem.words(3);
  await page.getByLabel("Title").type(randomTitle);

  const randomMarkdown = `
    # ${faker.lorem.words(3)}

    ${faker.lorem.paragraphs(3)}
  `;
  await page.getByLabel("Article Body").type(randomMarkdown);

  let called = false;
  await page.route(GRAPHQL_URI, async route => {
    const body = route.request().postData() || "";

    expect(JSON.parse(body)).toEqual(createArticleIntercept(randomMarkdown, randomTitle));
    called = true;
    const responseJson = createArticleMockData();

    return route.fulfill({json: responseJson});
  });
  await page.getByRole('button', {name: "Create Article"}).click();
  await expect(page.getByText("Created Article")).toBeVisible();
  expect(called).toBeTruthy();
});

test("An article cannot be created with a missing title", async ({page}) => {
  await login(Role.Author, page);
  await page.goto("/create_article");
  const randomMarkdown = `
    # ${faker.lorem.words(3)}

    ${faker.lorem.paragraphs(3)}
  `;
  await page.getByLabel("Article Body").type(randomMarkdown);

  let called = false;
  await page.route(GRAPHQL_URI, async route => {
    called = true;
  });

  await page.getByRole('button', {name: "Create Article"}).click();
  await expect(page.getByText("Articles must have a title")).toBeVisible();
  expect(called).toBeFalsy();
});

test("An article cannot be created with a missing content", async ({page}) => {
  await login(Role.Author, page);
  await page.goto("/create_article");

  const randomTitle = faker.lorem.words(3);
  await page.getByLabel("Title").type(randomTitle);

  let called = false;
  await page.route(GRAPHQL_URI, async route => {
    called = true;
  });

  await page.getByRole('button', {name: "Create Article"}).click();
  await expect(page.getByText("Articles must have content")).toBeVisible();
  expect(called).toBeFalsy();
});

// test("An learner cannot create an article", async ({page}) => {});

// test("An visitor cannot create an article", async ({page}) => {});

// test("An error message displays when there is an error creating an article", async ({page}) => {});
