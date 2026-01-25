<script setup lang="ts">
import { computed, ref } from "vue";

const { t } = useI18n();
const localePath = useLocalePath();
const route = useRoute();

const canonicalUrl = computed(() => `https://whis.ink${route.path}`);

useHead({
  title: t("cli.title"),
  link: [{ rel: "canonical", href: canonicalUrl }],
  meta: [
    { name: "description", content: t("cli.metaDescription") },
    { property: "og:title", content: t("cli.title") },
    { property: "og:description", content: t("cli.metaDescription") },
    { property: "og:url", content: canonicalUrl },
    { property: "og:image", content: "https://whis.ink/og-image.jpg" },
    { property: "og:type", content: "website" },
    { name: "twitter:card", content: "summary_large_image" },
    { name: "twitter:title", content: t("cli.title") },
    { name: "twitter:description", content: t("cli.metaDescription") },
    { name: "twitter:image", content: "https://whis.ink/og-image.jpg" },
  ],
});

const installTab = ref("cargo");
const lightboxOpen = ref(false);

const demoImage = computed(() => [
  {
    src: "/demo.gif",
    alt: t("cli.demo.altText"),
    caption: t("cli.demo.caption"),
  },
]);
</script>

<template>
  <div class="cli-content">
    <ViewHeader
      :title="$t('cli.title').replace(' - whis', '')"
      :subtitle="$t('cli.subtitle')"
    />

    <!-- Install -->
    <section id="install" class="install">
      <TabPanel
        v-model:selected="installTab"
        :tabs="[
          { value: 'cargo', label: $t('cli.install.tabs.cargo') },
          { value: 'aur', label: $t('cli.install.tabs.aur') },
          { value: 'source', label: $t('cli.install.tabs.source') },
        ]"
      >
        <div v-if="installTab === 'cargo'" class="panel">
          <CommandCopy
            :segments="[
              { text: 'cargo install ' },
              { text: 'whis', highlight: true },
            ]"
          />
        </div>
        <div v-else-if="installTab === 'aur'" class="panel">
          <CommandCopy
            :segments="[{ text: 'yay -S ' }, { text: 'whis', highlight: true }]"
          />
        </div>
        <div v-else class="panel">
          <CommandCopy
            :segments="[
              { text: 'git clone ' },
              { text: 'https://github.com/frankdierolf/whis', highlight: true },
              { text: ' && cd whis && ' },
              { text: 'cargo build --release', highlight: true },
            ]"
          />
        </div>
      </TabPanel>
      <p class="install-note">
        <NuxtLink :to="localePath('downloads')">
          {{ $t("cli.install.moreOptions") }}
        </NuxtLink>
      </p>
    </section>

    <!-- Features -->
    <section class="features">
      <div class="section-header">
        <h2>{{ $t("cli.features.title") }}</h2>
        <p>{{ $t("cli.features.subtitle") }}</p>
      </div>
      <ul>
        <li>
          <span class="marker">[*]</span>
          <div>
            <strong>{{ $t("cli.features.items.oneCommand.title") }}</strong>
            {{ $t("cli.features.items.oneCommand.description") }}
          </div>
        </li>
        <li>
          <span class="marker">[*]</span>
          <div>
            <strong>{{ $t("cli.features.items.providers.title") }}</strong>
            {{ $t("cli.features.items.providers.description") }}
          </div>
        </li>
        <li>
          <span class="marker">[*]</span>
          <div>
            <strong>{{ $t("cli.features.items.runLocally.title") }}</strong>
            {{ $t("cli.features.items.runLocally.description") }}
          </div>
        </li>
        <li>
          <span class="marker">[*]</span>
          <div>
            <strong>{{ $t("cli.features.items.postProcessing.title") }}</strong>
            {{ $t("cli.features.items.postProcessing.description") }}
          </div>
        </li>
        <li>
          <span class="marker">[*]</span>
          <div>
            <strong>{{ $t("cli.features.items.presets.title") }}</strong>
            {{ $t("cli.features.items.presets.description") }}
          </div>
        </li>
        <li>
          <span class="marker">[*]</span>
          <div>
            <strong>{{ $t("cli.features.items.hotkey.title") }}</strong>
            {{ $t("cli.features.items.hotkey.description") }}
          </div>
        </li>
      </ul>
    </section>

    <!-- Demo -->
    <section class="demo">
      <figure>
        <NuxtImg
          :src="demoImage[0]!.src"
          :alt="$t('cli.demo.altText')"
          width="800"
          format="webp"
          loading="lazy"
          class="clickable"
          @click="lightboxOpen = true"
        />
        <figcaption>{{ $t("cli.demo.caption") }}</figcaption>
      </figure>
    </section>

    <!-- Lightbox -->
    <Lightbox
      v-model:open="lightboxOpen"
      :images="demoImage"
      :initial-index="0"
    />

    <!-- Quick Start -->
    <section class="quickstart">
      <h2>{{ $t("cli.quickStart.title") }}</h2>
      <pre><code><span class="comment">{{ $t('cli.quickStart.comments.cloudSetup') }}</span>
<span class="highlight">whis setup cloud</span>

<span class="comment">{{ $t('cli.quickStart.comments.localSetup') }}</span>
<span class="highlight">whis setup local</span>

<span class="comment">{{ $t('cli.quickStart.comments.thenRun') }}</span>
<span class="highlight">whis</span>
<span class="comment">{{ $t('cli.quickStart.comments.pressEnter') }}</span>

<span class="comment">{{ $t('cli.quickStart.comments.postProcess') }}</span>
<span class="highlight">whis --post-process</span></code></pre>
    </section>
  </div>
</template>

<style scoped>
.cli-content {
  padding: 2rem;
}

.install {
  padding: 2rem 0;
}

.panel {
  display: block;
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

.demo img {
  width: 100%;
  height: auto;
  border-radius: 6px;
  border: 1px solid var(--border-weak);
}

.demo img.clickable {
  cursor: zoom-in;
  transition: border-color 0.15s ease;
}

.demo img.clickable:hover {
  border-color: var(--border);
}

.demo figcaption {
  font-size: 0.85rem;
  color: var(--text-weak);
  text-transform: uppercase;
  letter-spacing: 0.05em;
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
