export default defineI18nConfig(() => ({
  legacy: false,
  locale: 'en',
  fallbackLocale: 'en',
  // Allow HTML tags in translation messages (for FAQ answers with <code>, <strong>, <br>, etc.)
  // Note: Only use this with trusted content, as it can expose XSS vulnerabilities
  missingWarn: false,
  fallbackWarn: false,
}))
