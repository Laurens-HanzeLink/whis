import { expect, test } from '@playwright/test'

test('homepage has title and downloads link', async ({ page }) => {
  await page.goto('/')
  // Allow for title variations like "whis - Voice-to-text..."
  await expect(page).toHaveTitle(/whis/i)
  await expect(page.getByRole('link', { name: /download/i }).first()).toBeVisible()
})
