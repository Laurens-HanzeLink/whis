import { computed } from 'vue'
import { useCratesDownloads } from './useCratesDownloads'
import { useGitHubDownloads } from './useGitHubDownloads'
import { useFlathubDownloads } from './useFlathubDownloads'

export interface DownloadStats {
  total: number | null
  breakdown: {
    crates: number | null
    github: number | null
    flathub: number | null
  }
  loading: boolean
  hasError: boolean
}

export function useDownloadStats() {
  const crates = useCratesDownloads()
  const github = useGitHubDownloads()
  const flathub = useFlathubDownloads()

  const total = computed<number | null>(() => {
    const sources = [
      crates.count.value,
      github.count.value,
      flathub.count.value,
    ]

    // Filter out null values
    const validSources = sources.filter(v => v !== null) as number[]

    // Return null if no sources loaded successfully
    if (validSources.length === 0)
      return null

    // Sum all valid sources
    return validSources.reduce((sum, val) => sum + val, 0)
  })

  const loading = computed(() =>
    crates.loading.value || github.loading.value || flathub.loading.value,
  )

  const hasError = computed(() =>
    crates.error.value !== null
    || github.error.value !== null
    || flathub.error.value !== null,
  )

  const breakdown = computed(() => ({
    crates: crates.count.value,
    github: github.count.value,
    flathub: flathub.count.value,
  }))

  return {
    total,
    breakdown,
    loading,
    hasError,
  }
}
