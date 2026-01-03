<script setup lang="ts">
import { computed } from 'vue'

const { t } = useI18n()
const route = useRoute()

const canonicalUrl = computed(() => `https://whis.ink${route.path}`)

useHead({
  title: t('faq.title'),
  link: [
    { rel: 'canonical', href: canonicalUrl },
  ],
  meta: [
    { name: 'description', content: t('faq.metaDescription') },
    { property: 'og:title', content: t('faq.title') },
    { property: 'og:description', content: t('faq.metaDescription') },
    { property: 'og:url', content: canonicalUrl },
    { property: 'og:image', content: 'https://whis.ink/og-image.jpg' },
    { property: 'og:type', content: 'website' },
    { name: 'twitter:card', content: 'summary_large_image' },
    { name: 'twitter:title', content: t('faq.title') },
    { name: 'twitter:description', content: t('faq.metaDescription') },
    { name: 'twitter:image', content: 'https://whis.ink/og-image.jpg' },
  ],
})

const faqItems = computed(() => [
  {
    question: t('faq.items.postProcessing.question'),
    answer: 'faq.items.postProcessing.answer',
  },
  {
    question: t('faq.items.requirements.question'),
    answer: 'faq.items.requirements.answer',
  },
  {
    question: t('faq.items.privacy.question'),
    answer: 'faq.items.privacy.answer',
  },
  {
    question: t('faq.items.presets.question'),
    answer: 'faq.items.presets.answer',
  },
  {
    question: t('faq.items.configLocation.question'),
    answer: 'faq.items.configLocation.answer',
  },
  {
    question: t('faq.items.piping.question'),
    answer: 'faq.items.piping.answer',
  },
  {
    question: t('faq.items.uninstall.question'),
    answer: 'faq.items.uninstall.answer',
  },
])
</script>

<template>
  <div class="faq-content">
    <ViewHeader :title="$t('faq.title').replace(' - whis', '')" :subtitle="$t('faq.subtitle')" />

    <section class="faq">
      <FaqAccordion :items="faqItems" />
    </section>
  </div>
</template>

<style scoped>
.faq-content {
  padding: 2rem;
}

.faq {
  padding: var(--vertical-padding) var(--padding);
}
</style>
