package ink.whis.floatingbubble

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Intent
import android.graphics.Color
import android.graphics.PixelFormat
import android.graphics.drawable.GradientDrawable
import android.os.Build
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import android.util.Log
import android.view.Gravity
import android.view.MotionEvent
import android.view.View
import android.view.WindowManager
import android.widget.FrameLayout
import android.widget.ImageView
import androidx.core.app.NotificationCompat
import androidx.core.content.ContextCompat
import app.tauri.annotation.InvokeArg

/**
 * Foreground service that manages the floating bubble overlay.
 *
 * Uses standard Android WindowManager to create a draggable floating bubble.
 * Visual states change based on configured icons for each state.
 * Supports drag-to-close with a close zone at the bottom center.
 */
class FloatingBubbleService : Service() {

    companion object {
        private const val TAG = "FloatingBubbleService"
        private const val CHANNEL_ID = "floating_bubble_channel"
        private const val NOTIFICATION_ID = 1001
        private const val CLOSE_ZONE_SIZE = 80
        private const val CLOSE_ZONE_MARGIN = 16

        // Configuration passed from the plugin
        var bubbleSize: Int = 60
        var bubbleStartX: Int = 0
        var bubbleStartY: Int = 100
        var defaultIconResourceName: String? = null
        var backgroundColor: Int = Color.parseColor("#1C1C1C")
        var stateConfigs: Map<String, StateConfig> = emptyMap()

        // Reference to the current service instance for state updates
        @Volatile
        private var instance: FloatingBubbleService? = null

        // Store pending state when service isn't ready yet
        @Volatile
        private var pendingState: String? = null

        /**
         * Update the bubble's state from outside the service.
         * Runs on main thread to safely update UI.
         * If service isn't ready, stores the state for later application.
         */
        fun setState(state: String) {
            val service = instance
            Log.d(TAG, "setState called with: '$state', instance: $service, pendingState: $pendingState")
            Log.d(TAG, "setState - stateConfigs size: ${stateConfigs.size}, keys: ${stateConfigs.keys}")
            if (service == null) {
                // Store for later - will be applied when service starts
                Log.d(TAG, "Instance is null, storing pending state: $state")
                pendingState = state
                return
            }

            Log.d(TAG, "setState - posting updateState to main thread")
            Handler(Looper.getMainLooper()).post {
                service.updateState(state)
            }
        }

        /**
         * Reset static state when service is fully destroyed.
         */
        fun resetState() {
            pendingState = null
        }
    }

    private var windowManager: WindowManager? = null
    private var bubbleView: ImageView? = null
    private var bubbleBackground: GradientDrawable? = null
    private var layoutParams: WindowManager.LayoutParams? = null
    private var closeZoneParams: WindowManager.LayoutParams? = null
    private var closeZoneView: FrameLayout? = null
    private var closeZoneIcon: ImageView? = null
    private var closeZoneBackground: GradientDrawable? = null
    private var currentStateName: String = "idle"
    private var closeZoneVisible = false
    private var closeZoneActivated = false

    override fun onBind(intent: Intent?): IBinder? = null

    override fun onCreate() {
        super.onCreate()
        instance = this

        createNotificationChannel()
        startForeground(NOTIFICATION_ID, createNotification())

        windowManager = getSystemService(WINDOW_SERVICE) as WindowManager
        createCloseZone()
        createBubble()
    }

    override fun onDestroy() {
        super.onDestroy()
        instance = null
        removeCloseZone()
        removeBubble()
        FloatingBubblePlugin.isBubbleVisible = false
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID,
                "Floating Bubble",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "Voice input bubble service"
                setShowBadge(false)
            }
            val notificationManager = getSystemService(NotificationManager::class.java)
            notificationManager.createNotificationChannel(channel)
        }
    }

    private fun createCloseZone() {
        val density = resources.displayMetrics.density
        val sizePx = (CLOSE_ZONE_SIZE * density).toInt()
        val marginPx = (CLOSE_ZONE_MARGIN * density).toInt()

        closeZoneBackground = GradientDrawable().apply {
            shape = GradientDrawable.OVAL
            setColor(Color.parseColor("#60CCCCCC"))
            setStroke((2 * density).toInt(), Color.parseColor("#CC999999"))
        }

        closeZoneView = FrameLayout(this).apply {
            visibility = View.GONE
            this.background = closeZoneBackground

            closeZoneIcon = ImageView(this@FloatingBubbleService).apply {
                setImageResource(android.R.drawable.ic_menu_close_clear_cancel)
                setColorFilter(Color.WHITE)
                val padding = (sizePx * 0.3).toInt()
                setPadding(padding, padding, padding, padding)
            }

            addView(closeZoneIcon, FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT,
                FrameLayout.LayoutParams.MATCH_PARENT
            ))
        }

        @Suppress("DEPRECATION")
        val windowType = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            WindowManager.LayoutParams.TYPE_APPLICATION_OVERLAY
        } else {
            WindowManager.LayoutParams.TYPE_PHONE
        }

        closeZoneParams = WindowManager.LayoutParams(
            sizePx,
            sizePx,
            windowType,
            WindowManager.LayoutParams.FLAG_NOT_FOCUSABLE or
                WindowManager.LayoutParams.FLAG_LAYOUT_NO_LIMITS,
            PixelFormat.TRANSLUCENT
        ).apply {
            gravity = Gravity.BOTTOM or Gravity.CENTER_HORIZONTAL
            y = marginPx + sizePx
        }

        windowManager?.addView(closeZoneView, closeZoneParams)
    }

    private fun removeCloseZone() {
        closeZoneView?.let {
            try {
                windowManager?.removeView(it)
            } catch (e: Exception) {
                Log.e(TAG, "Error removing close zone view", e)
            }
        }
        closeZoneView = null
    }

    private fun showCloseZone() {
        if (closeZoneVisible) return
        closeZoneVisible = true
        closeZoneView?.visibility = View.VISIBLE
    }

    private fun hideCloseZone() {
        if (!closeZoneVisible) return
        closeZoneVisible = false
        closeZoneActivated = false
        closeZoneView?.visibility = View.GONE
        closeZoneBackground?.setColor(Color.parseColor("#60CCCCCC"))
        closeZoneBackground?.setStroke((2 * resources.displayMetrics.density).toInt(), Color.parseColor("#CC999999"))
    }

    private fun updateCloseZoneFeedback(isClose: Boolean) {
        if (isClose != closeZoneActivated) {
            closeZoneActivated = isClose
            if (isClose) {
                closeZoneBackground?.setColor(Color.parseColor("#80FF4444"))
                closeZoneBackground?.setStroke((4 * resources.displayMetrics.density).toInt(), Color.parseColor("#FFFF4444"))
                closeZoneIcon?.setColorFilter(Color.parseColor("#FFFF4444"))
            } else {
                closeZoneBackground?.setColor(Color.parseColor("#60CCCCCC"))
                closeZoneBackground?.setStroke((2 * resources.displayMetrics.density).toInt(), Color.parseColor("#CC999999"))
                closeZoneIcon?.setColorFilter(Color.WHITE)
            }
        }
    }

    private fun createBubble() {
        val density = resources.displayMetrics.density
        val sizePx = (Companion.bubbleSize * density).toInt()
        val currentBackgroundColor = Companion.backgroundColor
        val currentIconResourceName = Companion.defaultIconResourceName

        // Create circular background with configured color
        bubbleBackground = GradientDrawable().apply {
            shape = GradientDrawable.OVAL
            setColor(currentBackgroundColor)
        }

        // Create bubble view with default icon
        bubbleView = ImageView(this).apply {
            background = bubbleBackground

            // Load icon by resource name, fallback to default
            val iconResId = if (!currentIconResourceName.isNullOrEmpty()) {
                resources.getIdentifier(
                    currentIconResourceName,
                    "drawable",
                    packageName
                )
            } else {
                0
            }

            if (iconResId != 0) {
                try {
                    val iconDrawable = ContextCompat.getDrawable(
                        this@FloatingBubbleService,
                        iconResId
                    )
                    setImageDrawable(iconDrawable)
                } catch (e: Exception) {
                    Log.e(TAG, "Failed to load icon: $currentIconResourceName", e)
                    loadDefaultIcon()
                }
            } else {
                // Try plugin's default icon, then fallback to system icon
                loadDefaultIcon()
            }

            scaleType = ImageView.ScaleType.CENTER_INSIDE
            val padding = (sizePx * 0.22).toInt()
            setPadding(padding, padding, padding, padding)

            contentDescription = "Floating bubble"
        }

        // Window layout params for overlay
        @Suppress("DEPRECATION")
        val windowType = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            WindowManager.LayoutParams.TYPE_APPLICATION_OVERLAY
        } else {
            WindowManager.LayoutParams.TYPE_PHONE
        }

        layoutParams = WindowManager.LayoutParams(
            sizePx,
            sizePx,
            windowType,
            WindowManager.LayoutParams.FLAG_NOT_FOCUSABLE or
                WindowManager.LayoutParams.FLAG_LAYOUT_NO_LIMITS,
            PixelFormat.TRANSLUCENT
        ).apply {
            gravity = Gravity.TOP or Gravity.START
            x = (Companion.bubbleStartX * density).toInt()
            y = (Companion.bubbleStartY * density).toInt()
        }

        // Add touch listener for dragging
        bubbleView?.setOnTouchListener(BubbleTouchListener())

        // Add to window
        windowManager?.addView(bubbleView, layoutParams)
        FloatingBubblePlugin.isBubbleVisible = true

        // Apply any pending state that was set before service was ready
        val pending = pendingState
        if (pending != null) {
            Log.d(TAG, "Applying pending state on bubble creation: '$pending'")
            pendingState = null
            updateState(pending)
        } else {
            // Apply initial idle state
            Log.d(TAG, "No pending state, applying initial IDLE")
            currentStateName = "idle"
            Log.d(TAG, "createBubble - Setting initial state to 'idle'")
        }
        Log.d(TAG, "createBubble - Initial stateConfigs size: ${Companion.stateConfigs.size}")
    }

    /**
     * Load the plugin's default icon or fallback to system icon.
     */
    private fun ImageView.loadDefaultIcon() {
        // Try plugin's default icon first
        val defaultResId = resources.getIdentifier(
            "ic_floating_bubble_default",
            "drawable",
            packageName
        )

        if (defaultResId != 0) {
            try {
                val defaultDrawable = ContextCompat.getDrawable(
                    this@FloatingBubbleService,
                    defaultResId
                )
                setImageDrawable(defaultDrawable)
                return
            } catch (e: Exception) {
                // Fall through to system icon
            }
        }

        // Fallback to system icon
        setImageResource(android.R.drawable.ic_btn_speak_now)
    }

    private fun removeBubble() {
        bubbleView?.let {
            try {
                windowManager?.removeView(it)
            } catch (e: Exception) {
                Log.e(TAG, "Error removing bubble view", e)
            }
        }
        bubbleView = null
    }

    /**
     * Touch listener that handles dragging the bubble.
     */
    private inner class BubbleTouchListener : View.OnTouchListener {

        private var initialX = 0
        private var initialY = 0
        private var initialTouchX = 0f
        private var initialTouchY = 0f
        private var isDragging = false
        private val clickThreshold = 10 // pixels

        override fun onTouch(view: View, event: MotionEvent): Boolean {
            when (event.action) {
                MotionEvent.ACTION_DOWN -> {
                    initialX = layoutParams?.x ?: 0
                    initialY = layoutParams?.y ?: 0
                    initialTouchX = event.rawX
                    initialTouchY = event.rawY
                    isDragging = false
                    showCloseZone()
                    return true
                }
                MotionEvent.ACTION_MOVE -> {
                    val deltaX = (event.rawX - initialTouchX).toInt()
                    val deltaY = (event.rawY - initialTouchY).toInt()

                    if (kotlin.math.abs(deltaX) > clickThreshold ||
                        kotlin.math.abs(deltaY) > clickThreshold) {
                        isDragging = true
                    }

                    layoutParams?.x = initialX + deltaX
                    layoutParams?.y = initialY + deltaY
                    windowManager?.updateViewLayout(bubbleView, layoutParams)

                    updateCloseZoneFeedback(isNearCloseZone())
                    return true
                }
                MotionEvent.ACTION_UP -> {
                    hideCloseZone()
                    if (!isDragging) {
                        handleBubbleClick()
                    } else {
                        if (isInCloseZone()) {
                            handleCloseBubble()
                        } else {
                            animateToEdge()
                        }
                    }
                    return true
                }
            }
            return false
        }

        private fun isInCloseZone(): Boolean {
            val bubble = bubbleView ?: return false
            val closeZone = closeZoneView ?: return false

            val bubbleLocation = IntArray(2)
            val closeZoneLocation = IntArray(2)
            bubble.getLocationOnScreen(bubbleLocation)
            closeZone.getLocationOnScreen(closeZoneLocation)

            val bubbleCenterX = bubbleLocation[0] + bubble.width / 2
            val bubbleCenterY = bubbleLocation[1] + bubble.height / 2
            val closeZoneCenterX = closeZoneLocation[0] + closeZone.width / 2
            val closeZoneCenterY = closeZoneLocation[1] + closeZone.height / 2

            val distance = kotlin.math.sqrt(
                Math.pow((bubbleCenterX - closeZoneCenterX).toDouble(), 2.0) +
                Math.pow((bubbleCenterY - closeZoneCenterY).toDouble(), 2.0)
            )

            val maxDistance = (closeZone.width / 2 + bubble.width / 2) * 0.7

            Log.d(TAG, "isInCloseZone: bubbleScreen=($bubbleCenterX, $bubbleCenterY), closeZoneScreen=($closeZoneCenterX, $closeZoneCenterY), distance=$distance, maxDistance=$maxDistance")

            return distance < maxDistance
        }

        private fun isNearCloseZone(): Boolean {
            val bubble = bubbleView ?: return false
            val closeZone = closeZoneView ?: return false

            val bubbleLocation = IntArray(2)
            val closeZoneLocation = IntArray(2)
            bubble.getLocationOnScreen(bubbleLocation)
            closeZone.getLocationOnScreen(closeZoneLocation)

            val bubbleCenterX = bubbleLocation[0] + bubble.width / 2
            val bubbleCenterY = bubbleLocation[1] + bubble.height / 2
            val closeZoneCenterX = closeZoneLocation[0] + closeZone.width / 2
            val closeZoneCenterY = closeZoneLocation[1] + closeZone.height / 2

            val distance = kotlin.math.sqrt(
                Math.pow((bubbleCenterX - closeZoneCenterX).toDouble(), 2.0) +
                Math.pow((bubbleCenterY - closeZoneCenterY).toDouble(), 2.0)
            )

            val nearDistance = (closeZone.width / 2 + bubble.width / 2) * 1.2

            return distance < nearDistance
        }
    }

    private fun handleBubbleClick() {
        FloatingBubblePlugin.sendBubbleClickEvent()
    }

    private fun handleCloseBubble() {
        FloatingBubblePlugin.sendCloseEvent()
        hideBubble()
    }

    private fun hideBubble() {
        try {
            val intent = Intent(this, FloatingBubbleService::class.java)
            stopService(intent)
        } catch (e: Exception) {
            Log.e(TAG, "Error hiding bubble", e)
        }
    }

    private fun animateToEdge() {
        val screenWidth = resources.displayMetrics.widthPixels
        val bubbleWidth = bubbleView?.width ?: 0
        val currentX = layoutParams?.x ?: 0

        val targetX = if (currentX + bubbleWidth / 2 < screenWidth / 2) {
            0
        } else {
            screenWidth - bubbleWidth
        }

        layoutParams?.x = targetX
        windowManager?.updateViewLayout(bubbleView, layoutParams)
    }

    /**
     * Update the visual state of the bubble.
     * Changes the icon based on state configuration.
     */
    private fun updateState(stateName: String) {
        Log.d(TAG, "updateState called: '$stateName', current: '$currentStateName'")
        Log.d(TAG, "updateState - bubbleView exists: ${bubbleView != null}")
        Log.d(TAG, "updateState - packageName: $packageName")

        if (currentStateName == stateName) {
            Log.d(TAG, "State unchanged, skipping update")
            return
        }
        currentStateName = stateName

        // Get state configuration
        val config = Companion.stateConfigs[stateName]
        Log.d(TAG, "updateState - State config for '$stateName': $config")
        Log.d(TAG, "updateState - stateConfigs keys: ${Companion.stateConfigs.keys}")
        Log.d(TAG, "updateState - stateConfigs values: ${Companion.stateConfigs.values}")

        // Determine icon: state-specific icon -> default icon -> system fallback
        val iconName = config?.iconResourceName ?: Companion.defaultIconResourceName
        Log.d(TAG, "updateState - Resolved icon name: '$iconName'")

        if (iconName != null) {
            // Load and set state-specific icon
            val iconResId = resources.getIdentifier(iconName, "drawable", packageName)
            Log.d(TAG, "updateState - Looking up resource '$iconName' in package '$packageName', resId: $iconResId")

            if (iconResId != 0) {
                try {
                    val iconDrawable = ContextCompat.getDrawable(this, iconResId)
                    Log.d(TAG, "updateState - Got drawable for '$iconName': ${iconDrawable != null}")
                    bubbleView?.setImageDrawable(iconDrawable)
                    Log.d(TAG, "updateState - SUCCESS: Loaded and set state icon: $iconName (resId: $iconResId)")
                } catch (e: Exception) {
                    Log.e(TAG, "updateState - FAILED: Exception loading state icon: $iconName", e)
                }
            } else {
                Log.w(TAG, "updateState - FAILED: State icon resource not found: $iconName")
                Log.d(TAG, "updateState - Listing available drawable resources:")
                try {
                    val fields = R.drawable::class.java.fields
                    for (field in fields) {
                        if (field.name.contains("whis", ignoreCase = true)) {
                            Log.d(TAG, "updateState - Available drawable: ${field.name}")
                        }
                    }
                } catch (e: Exception) {
                    Log.e(TAG, "updateState - Failed to list drawables", e)
                }
            }
        } else {
            Log.w(TAG, "updateState - No icon name resolved for state '$stateName'")
        }

        // Update notification
        val notificationManager = getSystemService(NotificationManager::class.java)
        notificationManager.notify(NOTIFICATION_ID, createNotification())
    }

    private fun createNotification(): Notification {
        val (title, text) = when (currentStateName.lowercase()) {
            "recording" -> "Recording..." to "Tap bubble to stop"
            "processing" -> "Processing..." to "Transcribing your voice"
            else -> "Floating Bubble" to "Tap the bubble to interact"
        }

        return NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle(title)
            .setContentText(text)
            .setSmallIcon(android.R.drawable.ic_btn_speak_now)
            .setPriority(NotificationCompat.PRIORITY_LOW)
            .setOngoing(true)
            .build()
    }
}
