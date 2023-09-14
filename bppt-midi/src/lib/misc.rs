pub(crate) fn bpm_to_tempo(bpm: u16) -> u32 {
    60_000_000 / (bpm as u32)
}
