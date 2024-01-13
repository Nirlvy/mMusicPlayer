type AudioFormat = { bits: number; chans: number; rate: number }
type MpdDuration = { nanos: number; secs: number }
type QueuePlace = { id: number; pos: number; prio: number }
type ReplayGain = 'off' | 'track' | 'album' | 'auto'
type State = 'stop' | 'play' | 'pause'
type Range = [MpdDuration, MpdDuration?]

type setState = 'Next' | 'Prev' | 'Stop' | 'Toggle'

type MpdStatus = {
  audio: AudioFormat | null
  bitrate: number | null
  consume: boolean
  crossfade: MpdDuration | null
  duration: MpdDuration | null
  elapsed: MpdDuration | null
  error: string | null
  mixrampdb: number | null
  mixrampdelay: MpdDuration | null
  nextsong: QueuePlace | null
  queue_len: number
  queue_version: number
  random: boolean
  repeat: boolean
  replaygain: ReplayGain | null
  single: boolean
  song: QueuePlace | null
  state: State
  time: [MpdDuration, MpdDuration] | null
  updating_db: number | null
  volume: number
}

type Song = {
  artist: string | null
  duration: MpdDuration | null
  file: string
  last_mod: string | null
  name: string | null
  place: QueuePlace
  range: Range
  tags: Array<[string, string]>
  title: string | null
}
