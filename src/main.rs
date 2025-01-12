use std::{fs::File, io::BufWriter, path::PathBuf};

use clap::Parser;
use midly::{Header, TrackEvent};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    out: PathBuf,

    #[arg(short, long, default_value_t = 120)]
    bpm: i16,
}

fn main() {
    let args = Args::parse();

    let ticks_per_beat = 96;
    let header = Header {
        format: midly::Format::SingleTrack,
        timing: midly::Timing::Metrical(ticks_per_beat.into()),
    };

    let channel = 9.into();
    let key = 75.into();

    let click_on = TrackEvent {
        // todo: first one should be 0
        delta: 96.into(),
        kind: midly::TrackEventKind::Midi {
            channel,
            message: midly::MidiMessage::NoteOn {
                key,
                vel: 100.into(),
            },
        },
    };
    let end_of_track = TrackEvent {
        delta: 96.into(),
        kind: midly::TrackEventKind::Meta(midly::MetaMessage::EndOfTrack),
    };
    let mut track = vec![];

    for _ in 0..100 {
        track.push(&click_on);
    }
    track.push(&end_of_track);

    let tracks = vec![track];

    {
        let file = File::create(&args.out).expect("error creating output file");
        let mut writer = BufWriter::new(file);
    
        midly::write_std(&header, tracks, &mut writer).expect("error writing file");
    }
}
