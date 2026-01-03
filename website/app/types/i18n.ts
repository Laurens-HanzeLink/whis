// Centralized locale code definitions
export const LOCALE_CODES = ['en', 'zh', 'es', 'fr', 'de', 'pt', 'ru', 'ja', 'ko', 'it'] as const
export type LocaleCode = typeof LOCALE_CODES[number]
