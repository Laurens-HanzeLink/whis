import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

/**
 * Color configuration for bubble states.
 * All colors are hex strings (e.g., "#FF4444").
 */
export interface BubbleColors {
  /** Background color. Default: "#1C1C1C" (dark) */
  background?: string
  /** Icon color for idle state. Default: "#FFFFFF" (white) */
  idle?: string
  /** Icon color for recording state. Default: "#FF4444" (red) */
  recording?: string
  /** Icon color for processing state. Default: "#FFD633" (gold) */
  processing?: string
}

/**
 * Options for configuring the floating bubble.
 */
export interface BubbleOptions {
  /** Size of the bubble in dp. Default: 60 */
  size?: number
  /** Initial X position. Default: 0 */
  startX?: number
  /** Initial Y position. Default: 100 */
  startY?: number
  /**
   * Android drawable resource name for the icon (without "R.drawable." prefix).
   * If not specified, uses the plugin's default icon.
   * Example: "ic_my_app_logo"
   */
  iconResourceName?: string
  /** Color configuration for different bubble states. */
  colors?: BubbleColors
}

/**
 * Response from visibility check.
 */
export interface VisibilityResponse {
  visible: boolean
}

/**
 * Response from permission check.
 */
export interface PermissionResponse {
  granted: boolean
}

/**
 * Show the floating bubble overlay.
 *
 * @param options - Configuration options for the bubble
 * @throws If overlay permission is not granted
 *
 * @example
 * ```typescript
 * import { showBubble } from 'tauri-plugin-floating-bubble'
 *
 * // Basic usage with defaults
 * await showBubble()
 *
 * // With custom icon and colors
 * await showBubble({
 *   size: 60,
 *   startX: 0,
 *   startY: 200,
 *   iconResourceName: 'ic_my_logo',
 *   colors: {
 *     background: '#1C1C1C',
 *     idle: '#FFFFFF',
 *     recording: '#FF4444',
 *     processing: '#FFD633'
 *   }
 * })
 * ```
 */
export async function showBubble(options?: BubbleOptions): Promise<void> {
  await invoke('plugin:floating-bubble|show_bubble', { options })
}

/**
 * Hide the floating bubble overlay.
 *
 * @example
 * ```typescript
 * import { hideBubble } from 'tauri-plugin-floating-bubble'
 * await hideBubble()
 * ```
 */
export async function hideBubble(): Promise<void> {
  await invoke('plugin:floating-bubble|hide_bubble')
}

/**
 * Check if the floating bubble is currently visible.
 *
 * @returns Whether the bubble is visible
 *
 * @example
 * ```typescript
 * import { isBubbleVisible } from 'tauri-plugin-floating-bubble'
 * const { visible } = await isBubbleVisible()
 * ```
 */
export async function isBubbleVisible(): Promise<VisibilityResponse> {
  return await invoke<VisibilityResponse>('plugin:floating-bubble|is_bubble_visible')
}

/**
 * Request the overlay permission (SYSTEM_ALERT_WINDOW).
 * Opens system settings if permission is not granted.
 *
 * @returns Whether permission was granted
 *
 * @example
 * ```typescript
 * import { requestOverlayPermission } from 'tauri-plugin-floating-bubble'
 * const { granted } = await requestOverlayPermission()
 * if (granted) {
 *   await showBubble()
 * }
 * ```
 */
export async function requestOverlayPermission(): Promise<PermissionResponse> {
  return await invoke<PermissionResponse>('plugin:floating-bubble|request_overlay_permission')
}

/**
 * Check if the overlay permission (SYSTEM_ALERT_WINDOW) is granted.
 *
 * @returns Whether permission is granted
 *
 * @example
 * ```typescript
 * import { hasOverlayPermission } from 'tauri-plugin-floating-bubble'
 * const { granted } = await hasOverlayPermission()
 * ```
 */
export async function hasOverlayPermission(): Promise<PermissionResponse> {
  return await invoke<PermissionResponse>('plugin:floating-bubble|has_overlay_permission')
}

/**
 * Bubble visual state.
 * - idle: Default state with idle color
 * - recording: Active recording state with recording color
 * - processing: Processing/transcribing state with processing color
 */
export type BubbleState = 'idle' | 'recording' | 'processing'

/**
 * Update the bubble's visual state.
 *
 * @param state - The visual state to apply
 *
 * @example
 * ```typescript
 * import { setBubbleState } from 'tauri-plugin-floating-bubble'
 * await setBubbleState('idle')       // Default state
 * await setBubbleState('recording')  // Recording state
 * await setBubbleState('processing') // Processing state
 * ```
 */
export async function setBubbleState(state: BubbleState): Promise<void> {
  await invoke('plugin:floating-bubble|set_bubble_state', { state })
}

/**
 * Update the bubble's visual state to indicate recording.
 *
 * @deprecated Use setBubbleState() instead for more control
 * @param recording - Whether the bubble should show recording state
 *
 * @example
 * ```typescript
 * import { setBubbleRecording } from 'tauri-plugin-floating-bubble'
 * await setBubbleRecording(true) // Show recording state
 * await setBubbleRecording(false) // Show idle state
 * ```
 */
export async function setBubbleRecording(recording: boolean): Promise<void> {
  await setBubbleState(recording ? 'recording' : 'idle')
}

/**
 * Event payload when the bubble is clicked.
 */
export interface BubbleClickEvent {
  /** The action that triggered the event */
  action: 'click'
}

/**
 * The event name used for bubble click events.
 * Can be used with `listen()` from `@tauri-apps/api/event` directly.
 */
export const BUBBLE_CLICK_EVENT = 'floating-bubble://click'

/**
 * Register a listener for bubble click events.
 *
 * This uses Tauri's global event system (same pattern as official plugins like plugin-store).
 *
 * @param callback - Function to call when the bubble is clicked
 * @returns A function to unregister the listener
 *
 * @example
 * ```typescript
 * import { onBubbleClick } from 'tauri-plugin-floating-bubble'
 *
 * const unlisten = await onBubbleClick((event) => {
 *   console.log('Bubble clicked!', event.action)
 *   // Start/stop recording, etc.
 * })
 *
 * // Later, to stop listening:
 * unlisten()
 * ```
 */
export async function onBubbleClick(
  callback: (event: BubbleClickEvent) => void
): Promise<() => void> {
  return await listen<BubbleClickEvent>(BUBBLE_CLICK_EVENT, (event) => {
    callback(event.payload)
  })
}
