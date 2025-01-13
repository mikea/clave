use std::{fs::File, io::BufWriter, path::PathBuf};

use clap::Parser;
use midly::{Header, TrackEvent};
use pattern::PatternItem;

mod pattern;

#[derive(Parser, Debug)]
struct Args {
    /// Output file
    #[arg(short, long)]
    out: PathBuf,

    /// Track tempo (beats per minute)
    #[arg(short, long, default_value_t = 120)]
    bpm: u16,

    /// Midi channel
    #[arg(long, default_value_t = 10)]
    channel: u8,

    /// Track length in number of beats
    #[arg(long, default_value_t = 1_000)]
    beats: usize,

    /// Default note velocity
    #[arg(long, default_value_t = 63)]
    vel: u8,

    /// Accented note velocity
    #[arg(long, default_value_t = 94)]
    acc_vel: u8,

    /// Ghost note velocity
    #[arg(long, default_value_t = 31)]
    ghost_vel: u8,

    /// Number of subdivisions in a pattern
    #[arg(long, default_value_t = 1)]
    subs: u16,

    /// Click pattern
    #[arg(short, long, default_value_t = String::from("m>mmm"))]
    pattern: String,
}

fn main() {
    let args = Args::parse();
    args.main();
}

impl Args {
    fn main(&self) {
        assert!(self.channel > 0, "channel needs to be >0");
        let channel = self.channel - 1;
        let ticks_per_beat = self.subs;
        let delta = 1;

        let tempo = 60.0 * 1_000_000.0 / (self.bpm as f64);
        let tempo = tempo as u32;

        let header = Header {
            format: midly::Format::SingleTrack,
            timing: midly::Timing::Metrical(ticks_per_beat.into()),
        };

        let set_tempo = TrackEvent {
            delta: 0.into(),
            kind: midly::TrackEventKind::Meta(midly::MetaMessage::Tempo(tempo.into())),
        };

        let end_of_track = TrackEvent {
            delta: delta.into(),
            kind: midly::TrackEventKind::Meta(midly::MetaMessage::EndOfTrack),
        };

        let pattern = pattern::parse(&self.pattern);

        let mut events = vec![set_tempo];
        let mut d = 0;
        for i in 0..(self.beats * self.subs as usize) {
            let item = &pattern[i % pattern.len()];
            match item {
                PatternItem::Rest => d += delta,
                PatternItem::Note { key, vel } => {
                    events.push(TrackEvent {
                        delta: d.into(),
                        kind: midly::TrackEventKind::Midi {
                            channel: channel.into(),
                            message: midly::MidiMessage::NoteOn {
                                key: (*key).into(),
                                vel: self.vel(vel),
                            },
                        },
                    });
                    d = delta;
                }
            };
        }
        events.push(end_of_track);

        let mut track = Vec::with_capacity(events.len());
        for e in events.iter() {
            track.push(e)
        }
        let tracks = vec![track];

        {
            let file = File::create(&self.out).expect("error creating output file");
            let mut writer = BufWriter::new(file);
            midly::write_std(&header, tracks, &mut writer).expect("error writing file");
        }
    }

    fn vel(&self, vel: &pattern::Velocity) -> midly::num::u7 {
        match vel {
            pattern::Velocity::Default => self.vel.into(),
            pattern::Velocity::Accented => self.acc_vel.into(),
            pattern::Velocity::Ghost => self.ghost_vel.into(),
        }
    }
}
