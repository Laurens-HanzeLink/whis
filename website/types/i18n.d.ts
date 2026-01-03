/**
 * Type-safe i18n translation keys
 * Provides autocomplete for translation keys in IDE
 *
 * Based on Vue I18n TypeScript support:
 * https://vue-i18n.intlify.dev/guide/advanced/typescript
 */

import type en from '../i18n/locales/en.json'

declare module 'vue-i18n' {
  // Define the schema for translation messages
  export interface DefineLocaleMessage extends Record<string, any> {
    nav: typeof en.nav
    common: typeof en.common
    home: typeof en.home
    downloads: typeof en.downloads
    cli: typeof en.cli
    desktop: typeof en.desktop
    mobile: typeof en.mobile
    faq: typeof en.faq
  }
}
