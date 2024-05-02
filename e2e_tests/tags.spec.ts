import test, { expect } from "@playwright/test";
import {login, UserRole} from '../playwright/auth'

test('visitors cannot visit tags', async ({ page }) => {
  await page.goto('http://localhost:8080/');
  await expect(page.getByRole('link', { name: 'Login or Signup' })).toBeVisible();
  await page.goto('http://localhost:8080/tags');
  await expect(page.getByText('Only Authors can manage tags')).toBeVisible();
  expect(page.url()).not.toContain("tags");
});

test('learners cannot visit tags', async ({ page }) => {
  await login(page, UserRole.learner);
  await page.goto('http://localhost:8080/tags');
  await expect(page.getByText('Only Authors can manage tags')).toBeVisible();
  expect(page.url()).not.toContain("tags");
});
