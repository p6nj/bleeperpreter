use apres::{MIDIBytes, MIDIEvent, MIDI};

pub(super) fn as_bytes(midi: &MIDI) -> Vec<u8> {
    // First 8  bytes will always be the same
    let mut output: Vec<u8> = vec!['M' as u8, 'T' as u8, 'h' as u8, 'd' as u8, 0, 0, 0, 6];

    let format: u16 = midi.get_format();
    output.push((format / 256) as u8);
    output.push((format % 256) as u8);

    let track_count: u16 = midi.count_tracks() as u16;
    output.push((track_count / 256) as u8);
    output.push((track_count % 256) as u8);

    let ppqn: u16 = midi.get_ppqn();
    output.push((ppqn / 256) as u8);
    output.push((ppqn % 256) as u8);

    // Tracks (MTrk)
    let mut track_event_bytes: Vec<u8>;
    let mut track_byte_length: u32;
    let tracks: Vec<Vec<(usize, u64)>> = midi.get_tracks();

    for ticks in tracks.iter() {
        output.push('M' as u8);
        output.push('T' as u8);
        output.push('r' as u8);
        output.push('k' as u8);

        track_event_bytes = Vec::new();
        for (tick_delay, eid) in ticks.iter() {
            match midi.get_event(*eid) {
                Some(working_event) => {
                    track_event_bytes.extend(to_variable_length_bytes(*tick_delay).iter().copied());
                    track_event_bytes.extend(working_event.as_bytes());
                }
                None => {}
            }
        }

        // Automatically handle EndOfTrackEvent Here instead of requiring it be in the MIDITrack Object
        track_event_bytes.push(0);
        track_event_bytes.extend(MIDIEvent::EndOfTrack.as_bytes().iter().copied());

        // track length in bytes
        track_byte_length = track_event_bytes.len() as u32;
        output.push((track_byte_length / 256_u32.pow(3)) as u8);
        output.push(((track_byte_length / 256_u32.pow(2)) % 256) as u8);
        output.push(((track_byte_length / 256_u32.pow(1)) % 256) as u8);
        output.push((track_byte_length % 256) as u8);

        output.extend(track_event_bytes.iter().copied());
    }

    output
}

fn to_variable_length_bytes(number: usize) -> Vec<u8> {
    let mut output = Vec::new();
    let mut first_pass = true;
    let mut working_number = number;
    let mut tmp;
    while working_number > 0 || first_pass {
        tmp = working_number & 0x7F;
        working_number >>= 7;

        if !first_pass {
            tmp |= 0x80;
        }

        output.push(tmp as u8);
        first_pass = false;
    }
    output.reverse();

    output
}
