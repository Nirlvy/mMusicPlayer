type AudioFormat = { bits: number; chans: number; rate: number }
type Duration = { nanos: number; secs: number }
type QueuePlace = { id: number; pos: number; prio: number }
type ReplayGain = 'Off' | 'Track' | 'Album' | 'Auto'
type State = 'Stop' | 'Play' | 'Pause'

declare type MpdStatus = {
  audio: AudioFormat | null
  bitrate: number | null
  consume: boolean
  crossfade: Duration | null
  duration: Duration | null
  elapsed: Duration | null
  error: string | null
  mixrampdb: number | null
  mixrampdelay: null
  nextsong: QueuePlace
  queue_len: number
  queue_version: number
  random: boolean
  repeat: boolean
  replaygain: ReplayGain
  single: boolean
  song: QueuePlace
  state: State
  time: [Duration, Duration]
  updating_db: number | null
  volume: number
}
