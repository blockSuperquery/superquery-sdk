import { expect, test } from '@playwright/test'
import AxeBuilder from '@axe-core/playwright'

const routes = ['/', '/sdk', '/docs', '/examples', '/roadmap', '/status', '/changelog', '/grants', '/portal']

for (const route of routes) {
  test(`${route} renders without errors and is accessible`, async ({ page }) => {
    const errors: string[] = []
    page.on('pageerror', error => errors.push(error.message))
    page.on('console', (message) => {
      if (message.type() === 'error' || /hydration/i.test(message.text())) errors.push(message.text())
    })
    const response = await page.goto(route)
    expect(response?.status()).toBe(200)
    await expect(page.getByRole('heading', { level: 1 })).toBeVisible()
    await expect(page.getByRole('button', { name: /Switch to dark mode/ })).toBeEnabled()
    await page.evaluate(() => document.fonts.ready)
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true)
    expect(await page.locator('body').innerText()).not.toContain('\u2014')
    const links = await page.locator('a[href]').evaluateAll(elements => elements.map(element => element.getAttribute('href')).filter((href): href is string => !!href))
    for (const href of links.filter(href => href.startsWith('/') && !href.startsWith('//'))) {
      expect(routes).toContain(href.split('#')[0] || route)
    }
    const accessibility = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze()
    expect(accessibility.violations).toEqual([])
    expect(errors).toEqual([])
  })
}

test('mobile navigation opens, navigates, and closes', async ({ page, isMobile }) => {
  test.skip(!isMobile)
  await page.goto('/')
  await page.getByRole('button', { name: /Open menu/ }).click()
  const menu = page.getByRole('dialog')
  await expect(menu).toBeVisible()
  await menu.getByRole('link', { name: 'Documentation', exact: true }).click()
  await expect(page).toHaveURL(/\/docs$/)
  await expect(menu).not.toBeVisible()
})

test('code tabs respond to keyboard and copy the selected example', async ({ page, context }) => {
  await context.grantPermissions(['clipboard-read', 'clipboard-write'])
  await page.goto('/examples')
  const schema = page.getByRole('tab', { name: 'Schema', exact: true })
  await schema.click()
  await expect(page.getByRole('tabpanel').filter({ visible: true })).toContainText('type Transfer @entity')
  await page.getByRole('button', { name: 'Copy schema.graphql' }).click()
  await expect(page.getByRole('status').filter({ hasText: 'Code copied to clipboard' })).toBeVisible()
  expect(await page.evaluate(() => navigator.clipboard.readText())).toContain('type Transfer @entity')
  await schema.focus()
  await page.keyboard.press('ArrowRight')
  await expect(page.getByRole('tab', { name: 'GraphQL query' })).toBeFocused()
  await page.keyboard.press('Enter')
  await expect(page.getByRole('tabpanel').filter({ visible: true })).toContainText('pageInfo')
})

test('FAQ works with the keyboard and theme survives navigation', async ({ page }) => {
  await page.goto('/')
  const question = page.getByRole('button', { name: 'What can I use today?' })
  await question.focus()
  await page.keyboard.press('Enter')
  await expect(question).toHaveAttribute('aria-expanded', 'true')
  await expect(page.getByText('You can validate an EVM project', { exact: false })).toBeVisible()
  await page.getByRole('button', { name: /Switch to dark mode/ }).click()
  await expect(page.locator('html')).toHaveClass(/dark/)
  await page.goto('/docs')
  await expect(page.locator('html')).toHaveClass(/dark/)
  const accessibility = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21aa']).analyze()
  expect(accessibility.violations).toEqual([])
})

test('documentation anchors resolve to real sections', async ({ page }) => {
  await page.goto('/docs')
  for (const id of ['quickstart', 'project-files', 'handlers', 'query', 'specifications']) {
    await page.goto(`/docs#${id}`)
    await expect(page.locator(`#${id}`)).toBeInViewport()
  }
})

test('unknown URLs return a useful 404', async ({ page }) => {
  const response = await page.goto('/this-page-does-not-exist')
  expect(response?.status()).toBe(404)
  await expect(page.getByRole('heading', { name: 'This page is off the map.' })).toBeVisible()
  await page.getByRole('link', { name: 'Back to home' }).click()
  await expect(page).toHaveURL('/')
})
