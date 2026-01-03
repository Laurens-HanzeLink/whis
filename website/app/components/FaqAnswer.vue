<script setup lang="ts">
import DOMPurify from 'isomorphic-dompurify'
import { computed } from 'vue'

const props = defineProps<{
  answer: string
}>()

const { t } = useI18n()

// Sanitize HTML to only allow safe tags
const sanitizedAnswer = computed(() => {
  const rawHtml = t(props.answer)
  return DOMPurify.sanitize(rawHtml, {
    ALLOWED_TAGS: ['code', 'strong', 'br'],
    ALLOWED_ATTR: [],
  })
})
</script>

<template>
  <p class="faq-answer" v-html="sanitizedAnswer" />
</template>

<style scoped>
.faq-answer {
  line-height: 1.6;
}

.faq-answer :deep(code) {
  background: var(--bg-weak);
  padding: 0.125rem 0.375rem;
  border-radius: 3px;
  font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
  font-size: 0.9em;
  color: var(--accent);
}

.faq-answer :deep(strong) {
  font-weight: 600;
  color: var(--text-strong);
}
</style>
