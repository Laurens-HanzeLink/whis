<script setup lang="ts">
import type { LocaleCode } from '~/types/i18n'
import { computed, onMounted, onUnmounted, ref } from 'vue'

const { locale, locales } = useI18n()
const switchLocalePath = useSwitchLocalePath()
const isOpen = ref(false)
const buttonRef = ref<HTMLElement | null>(null)
const openUpward = ref(false)

const currentLocale = computed(() => locale.value.toUpperCase())

const availableLocales = computed(() => {
  return (locales.value as Array<{ code: string, name: string }>).map(loc => ({
    code: loc.code,
    label: loc.code.toUpperCase(),
    name: loc.name,
    path: switchLocalePath(loc.code as LocaleCode),
  }))
})

function toggleDropdown() {
  if (!isOpen.value && buttonRef.value) {
    // Calculate available space
    const rect = buttonRef.value.getBoundingClientRect()
    const spaceBelow = window.innerHeight - rect.bottom
    const spaceAbove = rect.top
    const dropdownHeight = 200 // max-height

    // Open upward if not enough space below
    openUpward.value = spaceBelow < dropdownHeight && spaceAbove > spaceBelow
  }
  isOpen.value = !isOpen.value
}

// Close on outside click and Escape key
onMounted(() => {
  const handleClickOutside = (e: MouseEvent) => {
    const target = e.target as HTMLElement
    if (!target.closest('.language-switcher')) {
      isOpen.value = false
    }
  }

  const handleKeydown = (e: KeyboardEvent) => {
    if (e.key === 'Escape') {
      isOpen.value = false
    }
  }

  document.addEventListener('click', handleClickOutside)
  document.addEventListener('keydown', handleKeydown)

  onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside)
    document.removeEventListener('keydown', handleKeydown)
  })
})
</script>

<template>
  <div class="language-switcher">
    <button
      ref="buttonRef"
      class="language-button"
      :aria-expanded="isOpen"
      aria-label="Switch language"
      @click="toggleDropdown"
    >
      <span class="language-value">{{ currentLocale }}</span>
      <span class="chevron" :class="{ open: isOpen, upward: openUpward }">›</span>
    </button>

    <Transition name="dropdown">
      <div v-if="isOpen" class="dropdown-menu" :class="{ 'open-upward': openUpward }">
        <NuxtLink
          v-for="loc in availableLocales"
          :key="loc.code"
          :to="loc.path"
          class="dropdown-item"
          :class="{ active: loc.code === locale }"
          @click="isOpen = false"
        >
          {{ loc.label }}
        </NuxtLink>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
/* Language switcher matching desktop AppSelect design */
.language-switcher {
  position: relative;
  width: 100%;
}

.language-button {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 8px 10px;
  background: var(--bg-weak);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text);
  font-family: var(--font);
  font-size: 12px;
  cursor: pointer;
  transition: border-color 0.15s ease;
}

.language-button:hover {
  border-color: var(--text-weak);
}

.language-button:focus {
  outline: none;
  border-color: var(--accent);
}

.language-switcher:has(.dropdown-menu) .language-button {
  border-color: var(--accent);
}

.language-value {
  flex: 1;
  text-align: left;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.chevron {
  transform: rotate(90deg);
  transition: transform 0.15s ease;
  color: var(--text-weak);
  font-size: 14px;
  margin-left: 8px;
}

.chevron.open {
  transform: rotate(-90deg);
}

.chevron.upward.open {
  transform: rotate(90deg);
}

.dropdown-menu {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 4px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  max-height: 200px;
  overflow-y: auto;
  z-index: 100;
}

.dropdown-menu.open-upward {
  top: auto;
  bottom: calc(100% + 4px);
}

.dropdown-item {
  display: block;
  padding: 8px 10px;
  color: var(--text);
  font-family: var(--font);
  font-size: 12px;
  text-decoration: none;
  transition: background 0.1s ease;
}

.dropdown-item:hover {
  background: var(--bg-weak);
}

.dropdown-item.active {
  color: var(--accent);
}

/* Transition */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

.dropdown-enter-from.open-upward,
.dropdown-leave-to.open-upward {
  transform: translateY(4px);
}
</style>
