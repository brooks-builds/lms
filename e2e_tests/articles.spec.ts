import {test, expect} from "@playwright/test";
import {faker} from "@faker-js/faker";
import {Role, login} from "./utils";

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

  await page.getByRole('button', {name: "Create Article"}).click();
  expect(page.getByText("Article Saved")).toBeVisible();
});

// test("An learner cannot create an article", async ({page}) => {});

// test("An visitor cannot create an article", async ({page}) => {});
