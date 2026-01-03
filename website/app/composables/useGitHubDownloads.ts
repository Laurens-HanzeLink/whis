import { onMounted, ref } from 'vue'

interface GitHubAsset {
  download_count: number
}

interface GitHubRelease {
  assets: GitHubAsset[]
}

export function useGitHubDownloads() {
  const count = ref<number | null>(null)
  const error = ref<Error | null>(null)
  const loading = ref(true)

  onMounted(async () => {
    try {
      const response = await fetch('https://api.github.com/repos/frankdierolf/whis/releases')
      if (response.ok) {
        const releases: GitHubRelease[] = await response.json()

        // Sum download_count from all assets across all releases
        const total = releases.reduce((sum, release) => {
          const releaseTotal = release.assets.reduce(
            (assetSum, asset) => assetSum + asset.download_count,
            0,
          )
          return sum + releaseTotal
        }, 0)

        count.value = total
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
