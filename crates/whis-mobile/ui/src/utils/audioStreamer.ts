/**
 * Stream audio from microphone in real-time
 *
 * Uses Web Audio API to capture raw samples at 16kHz mono
 * Emits chunks of f32 samples as they become available
 */

export interface AudioStreamerCallbacks {
  onChunk: (chunk: Float32Array) => void
  onError: (error: Error) => void
}

export class AudioStreamer {
  private stream: MediaStream | null = null
  private audioContext: AudioContext | null = null
  private scriptProcessor: ScriptProcessorNode | null = null
  private source: MediaStreamAudioSourceNode | null = null
  private isRecording = false

  private readonly TARGET_SAMPLE_RATE = 16000 // 16kHz
  private readonly CHANNELS = 1 // Mono
  private readonly CHUNK_SIZE = 4096 // ~256ms at 16kHz

  constructor(private callbacks: AudioStreamerCallbacks) {}

  async start(): Promise<void> {
    try {
      // Get microphone permission and stream
      this.stream = await navigator.mediaDevices.getUserMedia({
        audio: {
          echoCancellation: true,
          noiseSuppression: true,
          sampleRate: this.TARGET_SAMPLE_RATE,
        },
      })

      // Create audio context
      this.audioContext = new AudioContext({
        sampleRate: this.TARGET_SAMPLE_RATE,
      })

      // Create source from stream
      this.source = this.audioContext.createMediaStreamSource(this.stream)

      // Create script processor for raw samples
      // Note: ScriptProcessor is deprecated but most compatible
      this.scriptProcessor = this.audioContext.createScriptProcessor(
        this.CHUNK_SIZE,
        this.CHANNELS,
        this.CHANNELS,
      )

      // Process audio
      this.scriptProcessor.onaudioprocess = (event) => {
        const inputBuffer = event.inputBuffer
        const samples = inputBuffer.getChannelData(0) // Mono

        // Clone Float32Array (it's reused)
        const chunk = new Float32Array(samples)

        if (this.isRecording) {
          this.callbacks.onChunk(chunk)
        }
      }

      // Connect: Source → ScriptProcessor
      this.source.connect(this.scriptProcessor)
      this.scriptProcessor.connect(this.audioContext.destination)

      this.isRecording = true
    }
    catch (error) {
      this.callbacks.onError(error instanceof Error ? error : new Error(String(error)))
    }
  }

  stop(): void {
    this.isRecording = false

    // Disconnect nodes
    if (this.source) {
      this.source.disconnect()
      this.source = null
    }

    if (this.scriptProcessor) {
      this.scriptProcessor.disconnect()
      this.scriptProcessor = null
    }

    if (this.audioContext) {
      this.audioContext.close()
      this.audioContext = null
    }

    // Stop media stream
    if (this.stream) {
      this.stream.getTracks().forEach(track => track.stop())
      this.stream = null
    }
  }
}
