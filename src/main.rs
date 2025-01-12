use std::{fs::File, io::BufWriter, path::PathBuf};

use clap::Parser;
use midly::{Header, TrackEvent};
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    out: PathBuf,

    #[arg(short, long, default_value_t = 120)]
    bpm: u16,

    #[arg(short, long, default_value_t = String::from("m>mmm"))]
    pattern: String,

    #[arg(long, default_value_t = 10)]
    channel: u8,

    #[arg(long, default_value_t = 10_000)]
    beats: usize,

    #[arg(long, default_value_t = 63)]
    vel: u8,

    #[arg(long, default_value_t = 127)]
    acc_vel: u8,

    #[arg(long, default_value_t = 31)]
    ghost_vel: u8,

    #[arg(long, default_value_t = 1)]
    subs: u16,
}

fn main() {
    let args = Args::parse();
    args.main();
}

impl Args {
    fn main(&self) {
        assert!(self.channel > 0, "channel needs to be >0");
        let channel = (self.channel - 1) as u8;
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

        let pattern = self.parse_pattern();

        let mut events = vec![set_tempo];
        let mut d = 0;
        for i in 0..self.beats {
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
                                vel: (*vel).into(),
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

    fn parse_pattern(&self) -> Vec<PatternItem> {
        let re = Regex::new(r"[cmr][>]?").expect("bad regexp");
        let mut events = vec![];

        for cap in re.captures_iter(&self.pattern) {
            let cap = cap.get(0).unwrap().as_str();
            if cap.starts_with("r") {
                events.push(PatternItem::Rest);
                continue;
            }
            let key = match cap.chars().next().unwrap() {
                'm' => 32,
                'c' => 75,
                _ => unimplemented!("bad key: {cap:?}"),
            };
            let vel = if cap.ends_with(">") {
                self.acc_vel
            } else {
                self.vel
            };
            events.push(PatternItem::Note { key, vel });
        }
        assert!(!events.is_empty(), "bad pattern: {}", self.pattern);
        events
    }
}

enum PatternItem {
    Rest,
    Note { key: u8, vel: u8 },
}
