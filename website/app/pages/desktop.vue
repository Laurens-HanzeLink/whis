<script setup lang="ts">
import type { Platform } from '@/composables/usePlatformDetection'
import { computed, ref } from 'vue'

const { t } = useI18n()
const localePath = useLocalePath()
const route = useRoute()

const canonicalUrl = computed(() => `https://whis.ink${route.path}`)

useHead({
  title: t('desktop.title'),
  link: [
    { rel: 'canonical', href: canonicalUrl },
  ],
  meta: [
    { name: 'description', content: t('desktop.metaDescription') },
    { property: 'og:title', content: t('desktop.title') },
    { property: 'og:description', content: t('desktop.metaDescription') },
    { property: 'og:url', content: canonicalUrl },
    { property: 'og:image', content: 'https://whis.ink/og-image.jpg' },
    { property: 'og:type', content: 'website' },
    { name: 'twitter:card', content: 'summary_large_image' },
    { name: 'twitter:title', content: t('desktop.title') },
    { name: 'twitter:description', content: t('desktop.metaDescription') },
    { name: 'twitter:image', content: 'https://whis.ink/og-image.jpg' },
  ],
})

const { platform, arch } = usePlatformDetection()
const { version, versionNum, findAsset } = useGitHubRelease()

const lightboxOpen = ref(false)
const lightboxIndex = ref(0)

const screenshots = computed(() => [
  { src: '/screenshot-01-about.png', alt: 'About page showing Whis version and description', caption: t('desktop.screenshots.about') },
  { src: '/screenshot-02-tray.png', alt: 'Whis running in system tray', caption: t('desktop.screenshots.tray') },
  { src: '/screenshot-03-home.png', alt: 'Whis home screen with Start Recording button', caption: t('desktop.screenshots.home') },
  { src: '/screenshot-04-shortcuts.png', alt: 'Global shortcut configuration', caption: t('desktop.screenshots.shortcuts') },
  { src: '/screenshot-05-settings-cloud.png', alt: 'Cloud provider settings with OpenAI, Groq, Deepgram options', caption: t('desktop.screenshots.cloudMode') },
  { src: '/screenshot-06-settings-local.png', alt: 'Local whisper model selection for offline transcription', caption: t('desktop.screenshots.localMode') },
  { src: '/screenshot-07-presets.png', alt: 'Preset configurations for AI prompts, email, and notes', caption: t('desktop.screenshots.presets') },
])

const platformLabel = computed(() => {
  const labels: Record<Platform, string> = {
    linux: t('downloads.platforms.linux'),
    macos: t('downloads.platforms.macos'),
    windows: t('downloads.platforms.windows'),
    android: t('downloads.platforms.android'),
    unknown: t('downloads.platforms.unknown'),
  }
  return labels[platform.value]
})

const recommendedDownload = computed(() => {
  const v = version.value
  const vn = versionNum.value
  const base = `https://github.com/frankdierolf/whis/releases/download/${v}`

  switch (platform.value) {
    case 'linux':
      return {
        label: t('downloads.formats.appimage'),
        url: findAsset(/Whis_.*_amd64\.AppImage$/)?.browser_download_url || `${base}/Whis_${vn}_amd64.AppImage`,
      }
    case 'macos':
      return {
        label: t('downloads.formats.dmg'),
        url: arch.value === 'arm64'
          ? (findAsset(/Whis_.*_aarch64\.dmg$/)?.browser_download_url || `${base}/Whis_${vn}_aarch64.dmg`)
          : (findAsset(/Whis_.*_x64\.dmg$/)?.browser_download_url || `${base}/Whis_${vn}_x64.dmg`),
      }
    case 'windows':
      return {
        label: t('downloads.formats.exe'),
        url: findAsset(/Whis_.*_x64-setup\.exe$/)?.browser_download_url || `${base}/Whis_${vn}_x64-setup.exe`,
      }
    default:
      return {
        label: t('downloads.formats.appimage'),
        url: findAsset(/Whis_.*_amd64\.AppImage$/)?.browser_download_url || `${base}/Whis_${vn}_amd64.AppImage`,
      }
  }
})

function openLightbox(index: number) {
  lightboxIndex.value = index
  lightboxOpen.value = true
}
</script>

<template>
  <div class="desktop-content">
    <ViewHeader :title="$t('desktop.title').replace(' App - whis', '')" :subtitle="$t('desktop.subtitle')" />

    <!-- Install -->
    <section class="install">
      <h2 class="install-title">
        {{ $t('desktop.install.title', { platform: platformLabel }) }}
      </h2>
      <a :href="recommendedDownload.url" class="download-button">
        <span class="download-icon">↓</span>
        <span class="download-label">{{ recommendedDownload.label }}</span>
        <span class="download-version">{{ version }}</span>
      </a>
      <p class="install-note">
        <NuxtLink :to="localePath('downloads')">
          {{ $t('desktop.install.moreOptions') }}
        </NuxtLink>
      </p>
    </section>

    <!-- Features -->
    <section class="features">
      <div class="section-header">
        <h2>{{ $t('desktop.features.title') }}</h2>
        <p>{{ $t('desktop.features.subtitle') }}</p>
      </div>
      <ul>
        <li>
          <span class="marker">[*]</span>
          <div><strong>{{ $t('desktop.features.items.systemTray.title') }}</strong> {{ $t('desktop.features.items.systemTray.description') }}</div>
        </li>
        <li>
          <span class="marker">[*]</span>
          <div><strong>{{ $t('desktop.features.items.globalHotkey.title') }}</strong> {{ $t('desktop.features.items.globalHotkey.description') }}</div>
        </li>
        <li>
          <span class="marker">[*]</span>
          <div><strong>{{ $t('desktop.features.items.providers.title') }}</strong> {{ $t('desktop.features.items.providers.description') }}</div>
        </li>
        <li>
          <span class="marker">[*]</span>
          <div><strong>{{ $t('desktop.features.items.runLocally.title') }}</strong> {{ $t('desktop.features.items.runLocally.description') }}</div>
        </li>
        <li>
          <span class="marker">[*]</span>
          <div><strong>{{ $t('desktop.features.items.postProcessing.title') }}</strong> {{ $t('desktop.features.items.postProcessing.description') }}</div>
        </li>
        <li>
          <span class="marker">[*]</span>
          <div><strong>{{ $t('desktop.features.items.crossPlatform.title') }}</strong> {{ $t('desktop.features.items.crossPlatform.description') }}</div>
        </li>
      </ul>
    </section>

    <!-- Screenshots -->
    <section class="demo">
      <figure>
        <ImageCarousel :images="screenshots" @select="openLightbox" />
      </figure>
    </section>

    <!-- Quick Start -->
    <section class="quickstart">
      <h2>{{ $t('desktop.quickStart.title') }}</h2>
      <pre><code><span class="comment">{{ $t('desktop.quickStart.comments.download') }}</span>

<span class="comment">{{ $t('desktop.quickStart.comments.launch') }}</span>

<span class="comment">{{ $t('desktop.quickStart.comments.configure') }}</span>

<span class="comment">{{ $t('desktop.quickStart.comments.use') }}</span></code></pre>
    </section>

    <!-- Lightbox -->
    <Lightbox
      v-model:open="lightboxOpen"
      :images="screenshots"
      :initial-index="lightboxIndex"
    />
  </div>
</template>

<style scoped>
.desktop-content {
  padding: 2rem;
}

.install {
  padding: var(--vertical-padding) var(--padding);
}

.install-title {
  font-size: 0.9rem;
  font-weight: 500;
  color: var(--text-weak);
  margin-bottom: 0.75rem;
}

.download-button {
  display: inline-flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.875rem 1.5rem;
  background: var(--bg-strong);
  color: var(--text-inverted);
  border-radius: 4px;
  font-size: 0.95rem;
  font-weight: 600;
  text-decoration: none;
  transition: all 0.15s ease;
}

.download-button:hover {
  background: var(--bg-strong-hover);
  transform: translateX(2px);
}

.download-icon {
  font-size: 1.1rem;
}

.download-version {
  font-size: 0.8rem;
  font-weight: 400;
  opacity: 0.7;
}

.install-note {
  margin-top: 0.75rem;
  font-size: 0.75rem;
  color: var(--text-weak);
}

.install-note a {
  color: var(--text);
  text-decoration: underline;
  text-underline-offset: 2px;
}

.install-note a:hover {
  color: var(--accent);
}

.features {
  border-top: 1px solid var(--border-weak);
  padding: var(--vertical-padding) var(--padding);
}

.section-header {
  margin-bottom: 2rem;
}

.section-header h2 {
  font-size: 1.1rem;
  font-weight: 500;
  color: var(--text-strong);
  margin-bottom: 0.5rem;
}

.section-header p {
  color: var(--text);
}

.features ul {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.features li {
  display: flex;
  gap: 0.75rem;
  line-height: 1.6;
}

.marker {
  color: var(--icon);
  flex-shrink: 0;
}

.features li strong {
  color: var(--text-strong);
  font-weight: 500;
  margin-right: 0.5rem;
}

.demo {
  border-top: 1px solid var(--border-weak);
  padding: var(--vertical-padding) var(--padding);
}

.demo figure {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.quickstart {
  border-top: 1px solid var(--border-weak);
  padding: var(--vertical-padding) var(--padding);
}

.quickstart h2 {
  font-size: 1.1rem;
  font-weight: 500;
  color: var(--text-strong);
  margin-bottom: 1.5rem;
}

.quickstart pre {
  background: var(--bg-weak);
  border: 1px solid var(--border-weak);
  border-radius: 6px;
  padding: 1.5rem;
  overflow-x: auto;
}

.quickstart code {
  font-family: var(--font);
  font-size: 0.9rem;
  line-height: 1.8;
}

.comment {
  color: var(--text-weak);
}

.highlight {
  color: var(--text-strong);
  font-weight: 500;
}
</style>
