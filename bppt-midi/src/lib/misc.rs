pub fn bpm_to_tempo(bpm: usize) -> usize {
    60_000_000 / bpm
}
