import { reactive, readonly } from 'vue'

interface HeaderAction {
  label: string
  ariaLabel: string
  onClick: () => void
}

interface HeaderState {
  action: HeaderAction | null
}

const state = reactive<HeaderState>({
  action: null,
})

export const headerStore = {
  state: readonly(state),

  setAction(action: HeaderAction | null) {
    state.action = action
  },

  clearAction() {
    state.action = null
  },
}
