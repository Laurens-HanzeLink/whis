import { onMounted, ref } from 'vue'

interface CratesResponse {
  crate: {
    downloads: number
  }
}

export function useCratesDownloads() {
  const count = ref<number | null>(null)
  const error = ref<Error | null>(null)
  const loading = ref(true)

  onMounted(async () => {
    try {
      const response = await fetch('https://crates.io/api/v1/crates/whis')
      if (response.ok) {
        const data: CratesResponse = await response.json()
        count.value = data.crate.downloads
      }
    }
    catch (e) {
      error.value = e as Error
      // Silent fail - count remains null
    }
    finally {
      loading.value = false
    }
  })

  return { count, error, loading }
}
