# i18n Configuration

This directory contains internationalization (i18n) configuration for the whis website.

## Structure

```
i18n/
├── locales/          # Translation files for all supported languages
│   ├── en.json       # English (base locale, used for type generation)
│   ├── zh.json       # Chinese (Simplified)
│   ├── es.json       # Spanish
│   ├── fr.json       # French
│   ├── de.json       # German
│   ├── pt.json       # Portuguese
│   ├── ru.json       # Russian
│   ├── ja.json       # Japanese
│   ├── ko.json       # Korean
│   └── it.json       # Italian
├── i18n.config.ts    # Runtime Vue I18n configuration
└── README.md         # This file
```

## Supported Languages

- 🇬🇧 **English** (en) - Default
- 🇨🇳 **中文** (zh)
- 🇪🇸 **Español** (es)
- 🇫🇷 **Français** (fr)
- 🇩🇪 **Deutsch** (de)
- 🇵🇹 **Português** (pt)
- 🇷🇺 **Русский** (ru)
- 🇯🇵 **日本語** (ja)
- 🇰🇷 **한국어** (ko)
- 🇮🇹 **Italiano** (it)

## Type Safety

Translation keys are type-checked based on `en.json`. The project uses TypeScript definitions in `types/i18n.d.ts` to provide:

- ✅ **IDE autocomplete** for translation keys
- ✅ **Compile-time errors** for invalid keys
- ✅ **IntelliSense** showing available translations

### How It Works

1. The `en.json` file is the source of truth for translation structure
2. TypeScript types are generated from `en.json` in `types/i18n.d.ts`
3. All `t('...')` calls are type-checked against these definitions

### Example Usage

```vue
<script setup>
const { t } = useI18n()

// ✅ Valid - autocomplete will suggest these
const title = t('home.title')
const subtitle = t('home.subtitle')

// ❌ Invalid - TypeScript error
const invalid = t('home.doesNotExist')
</script>

<template>
  <h1>{{ $t('home.tagline.speak') }}</h1>
  <p>{{ $t('nav.home') }}</p>
</template>
```

## Adding New Translation Keys

1. **Add to `en.json` first** (base locale):
   ```json
   {
     "home": {
       "newKey": "New translation"
     }
   }
   ```

2. **Add to all other locale files** (`zh.json`, `es.json`, etc.):
   ```json
   {
     "home": {
       "newKey": "Nouvelle traduction"
     }
   }
   ```

3. **Restart dev server** to update types:
   ```bash
   npm run dev
   ```

4. **Use in components**:
   ```vue
   {{ $t('home.newKey') }}
   ```

## Adding a New Locale

1. **Create translation file**: `i18n/locales/xx.json` (copy from `en.json`)
2. **Translate all strings** in the new file
3. **Update `nuxt.config.ts`**:
   ```typescript
   i18n: {
     locales: [
       // ... existing locales
       { code: 'xx', iso: 'xx-XX', name: 'Language Name', file: 'xx.json', dir: 'ltr' }
     ]
   }
   ```
4. **Restart dev server**

## Translation File Format

Each translation file is a nested JSON structure:

```json
{
  "nav": {
    "home": "Home",
    "downloads": "Downloads"
  },
  "common": {
    "version": "v{version}",
    "license": "MIT"
  }
}
```

### Special Features

- **Variable interpolation**: Use `{variableName}` for dynamic values
  ```json
  "stars": "★ {count} stars"
  ```

- **HTML content**: Enabled for FAQ answers (use with caution)
  ```json
  "answer": "<strong>Important:</strong> Details here"
  ```

## Testing Translations

### Verify All Keys Match

```bash
# Run type check
npm run typecheck
```

### Visual Testing

1. Start dev server: `npm run dev`
2. Test each language using the language switcher
3. Check all pages: `/`, `/downloads`, `/cli`, `/desktop`, `/mobile`, `/faq`

## Troubleshooting

### Autocomplete Not Working

1. Restart TypeScript server in your IDE
2. Run `npx nuxi prepare` to regenerate types
3. Verify `types/i18n.d.ts` exists

### Missing Translations

- Check browser console for missing key warnings
- Verify key exists in `en.json`
- Ensure all locale files have matching structure

## Related Files

- **Configuration**: `nuxt.config.ts` (i18n module setup)
- **Runtime config**: `i18n.config.ts`
- **Type definitions**: `types/i18n.d.ts`
- **Language switcher**: `app/components/LanguageSwitcher.vue`

## Resources

- [@nuxtjs/i18n Documentation](https://i18n.nuxtjs.org/)
- [Vue I18n TypeScript Support](https://vue-i18n.intlify.dev/guide/advanced/typescript)
