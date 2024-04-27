import { test, expect } from '@playwright/test';
import { UserRole, login } from '../playwright/auth'

test('author can log in', async ({ page }) => {
  await login(page, UserRole.author)
});

test('learner can log in', async ({ page }) => {
  await login(page, UserRole.learner)
})
