<script setup lang="ts">
import { computed, onMounted, ref } from "vue";

const { t } = useI18n();
const localePath = useLocalePath();
const route = useRoute();

const canonicalUrl = computed(() => `https://whis.ink${route.path}`);

useHead({
  title: t("home.title"),
  link: [{ rel: "canonical", href: canonicalUrl }],
  meta: [
    { name: "description", content: t("home.metaDescription") },
    { property: "og:title", content: t("home.title") },
    { property: "og:description", content: t("home.metaDescription") },
    { property: "og:url", content: canonicalUrl },
    { property: "og:image", content: "https://whis.ink/og-image.jpg" },
    { property: "og:type", content: "website" },
    { name: "twitter:card", content: "summary_large_image" },
    { name: "twitter:title", content: t("home.title") },
    { name: "twitter:description", content: t("home.metaDescription") },
    { name: "twitter:image", content: "https://whis.ink/og-image.jpg" },
  ],
});

const stars = ref<number | null>(null);
const { total: downloads } = useDownloadStats();
const contributors = ref<{ login: string; avatar_url: string }[]>([]);

onMounted(async () => {
  // Fetch GitHub stars
  try {
    const gh = await fetch("https://api.github.com/repos/frankdierolf/whis");
    if (gh.ok) {
      const data = await gh.json();
      stars.value = data.stargazers_count;
    }
  } catch {
    /* silent fail */
  }

  // Fetch contributors
  try {
    const contribs = await fetch(
      "https://api.github.com/repos/frankdierolf/whis/contributors",
    );
    if (contribs.ok) {
      contributors.value = await contribs.json();
    }
  } catch {
    /* silent fail */
  }
});

function formatNumber(n: number): string {
  if (n >= 1000) return `${(n / 1000).toFixed(1)}k`;
  return n.toString();
}
</script>

<template>
  <div class="home">
    <div class="hero">
      <h1 class="tagline">
        <span>{{ $t("home.tagline.speak") }}</span>
        <span>{{ $t("home.tagline.paste") }}</span>
        <span>{{ $t("home.tagline.ship") }}</span>
      </h1>
      <p class="subtitle">
        {{ $t("home.subtitle") }}
      </p>
      <p class="description">
        {{ $t("home.description") }}
      </p>
    </div>

    <div class="cta-group">
      <NuxtLink :to="localePath('downloads')" class="cta-primary">
        <span class="cta-icon">↓</span>
        {{ $t("home.cta.download") }}
      </NuxtLink>
      <a
        href="https://github.com/frankdierolf/whis"
        target="_blank"
        rel="noopener"
        class="cta-secondary"
      >
        {{ $t("home.cta.github") }}
      </a>
    </div>

    <div class="proof">
      <a
        href="https://github.com/frankdierolf/whis"
        target="_blank"
        rel="noopener"
        class="stat stat-link"
      >
        {{ $t("home.proof.stars", { count: stars ?? "—" }) }}
      </a>
      <span class="divider">·</span>
      <a
        href="https://github.com/frankdierolf/whis/tree/main/website#notes"
        target="_blank"
        rel="noopener"
        class="stat stat-link"
      >
        {{
          $t("home.proof.installs", {
            count: downloads ? formatNumber(downloads) : "—",
          })
        }}
      </a>
      <span class="divider">·</span>
      <a
        v-if="contributors.length"
        href="https://github.com/frankdierolf/whis/graphs/contributors"
        target="_blank"
        rel="noopener"
        class="stat stat-link contributors"
      >
        <span class="avatars">
          <NuxtImg
            v-for="c in contributors"
            :key="c.login"
            :src="c.avatar_url"
            :alt="c.login"
            width="20"
            height="20"
            format="webp"
            loading="lazy"
          />
        </span>
        {{ $t("home.proof.contributors", { count: contributors.length }) }}
      </a>
      <span v-if="contributors.length" class="divider">·</span>
      <span class="stat">{{ $t("home.proof.license") }}</span>
    </div>

    <ClientOnly>
      <TerminalDemo />
    </ClientOnly>
  </div>
</template>

<style scoped>
.home {
  display: flex;
  flex-direction: column;
  justify-content: center;
  min-height: 100%;
  padding: 3rem;
  gap: 2.5rem;
}

.hero {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.tagline {
  font-size: clamp(2rem, 5vw, 3rem);
  font-weight: 700;
  color: var(--accent);
  letter-spacing: -0.03em;
  line-height: 1.1;
  display: flex;
  gap: 0.4em;
}

.tagline span {
  display: inline-block;
}

@keyframes reveal {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.subtitle {
  font-size: 1.25rem;
  color: var(--text-strong);
  font-weight: 500;
}

.description {
  font-size: 0.95rem;
  color: var(--text);
  max-width: 28rem;
  line-height: 1.6;
}

.cta-group {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

.cta-primary {
  display: inline-flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1.5rem;
  background: var(--bg-strong);
  color: var(--text-inverted);
  border: none;
  border-radius: 4px;
  font-family: var(--font);
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  text-decoration: none;
  transition: all 0.15s ease;
}

.cta-primary:hover {
  background: var(--bg-strong-hover);
  transform: translateX(2px);
}

.cta-icon {
  font-size: 1.1rem;
}

.cta-secondary {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  background: transparent;
  color: var(--text);
  border: 1px solid var(--border);
  border-radius: 4px;
  font-size: 0.9rem;
  font-weight: 500;
  text-decoration: none;
  transition: all 0.15s ease;
}

.cta-secondary:hover {
  background: var(--bg-weak);
  border-color: var(--text-weak);
  color: var(--text-strong);
}

.proof {
  display: flex;
  gap: 0.75rem;
  font-size: 0.8rem;
  color: var(--text-weak);
}

.stat {
  white-space: nowrap;
}

.divider {
  opacity: 0.4;
}

.contributors {
  display: inline-flex;
  align-items: center;
  gap: 0.4rem;
}

.avatars {
  display: inline-flex;
}

.avatars img {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  border: 2px solid var(--bg);
  margin-left: -8px;
}

.avatars img:first-child {
  margin-left: 0;
}

.stat-link {
  color: var(--text-weak);
  text-decoration: none;
  cursor: pointer;
  transition: color 0.15s ease;
}

.stat-link:hover {
  color: var(--accent);
}

.stat-link:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: 2px;
  border-radius: 2px;
}

@media (max-width: 768px) {
  .home {
    padding: 2rem;
    gap: 2rem;
  }

  .tagline {
    font-size: 1.75rem;
    flex-wrap: wrap;
    gap: 0.2em;
  }

  .proof {
    flex-wrap: wrap;
  }
}
</style>
