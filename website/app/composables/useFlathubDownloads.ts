import { onMounted, ref } from 'vue'

interface FlathubStats {
  installs_total: number
  installs_last_month?: number
  installs_last_7_days?: number
}

export function useFlathubDownloads() {
  const count = ref<number | null>(null)
  const error = ref<Error | null>(null)
  const loading = ref(true)

  onMounted(async () => {
    try {
      const response = await fetch('https://flathub.org/api/v2/stats/ink.whis.Whis')
      if (response.ok) {
        const data: FlathubStats = await response.json()
        count.value = data.installs_total
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
