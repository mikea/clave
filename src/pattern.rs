use chumsky::{error::Cheap, prelude::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PatternItem {
    Rest,
    Note { key: u8, vel: Velocity },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Velocity {
    Default,
    Accented,
    Ghost,
}

pub(crate) fn parse(s: &str) -> Vec<PatternItem> {
    parser().parse(s).expect("bad pattern")
}

fn parser() -> impl Parser<char, Vec<PatternItem>, Error = Cheap<char>> {
    // key 
    let m = just('m').to(32);
    let c = just('c').to(75);
    let h = just('h').to(42);
    let key = choice((m, c, h));

    // velocity
    let acc = just('>').to(Velocity::Accented);
    let ghost = just(',').to(Velocity::Ghost);
    let vel = acc.or(ghost).or_else(|_| Ok(Velocity::Default));

    // note
    let note = key
        .then(vel)
        .map(|(key, vel)| PatternItem::Note { key, vel });

    // rest
    let rest = just('r').to(PatternItem::Rest);

    // item and pattern
    let item = choice((rest, note));
    item.padded().repeated().collect()
}

#[cfg(test)]
mod test {

    use crate::pattern::{parse, PatternItem};

    #[test]
    fn parse_pattern() {
        assert_eq!(
            &[PatternItem::Rest, PatternItem::Rest, PatternItem::Rest],
            parse("rrr").as_slice()
        );
        assert_eq!(
            &[
                PatternItem::Note {
                    key: 32,
                    vel: crate::pattern::Velocity::Accented
                },
                PatternItem::Note {
                    key: 32,
                    vel: crate::pattern::Velocity::Default
                },
                PatternItem::Note {
                    key: 75,
                    vel: crate::pattern::Velocity::Ghost
                },
                PatternItem::Rest,
                PatternItem::Note {
                    key: 42,
                    vel: crate::pattern::Velocity::Default
                }
            ],
            parse("m>mc,rh").as_slice()
        );

        assert_eq!(
            &[
                PatternItem::Note {
                    key: 32,
                    vel: crate::pattern::Velocity::Accented
                },
                PatternItem::Note {
                    key: 32,
                    vel: crate::pattern::Velocity::Default
                },
                PatternItem::Note {
                    key: 75,
                    vel: crate::pattern::Velocity::Ghost
                },
                PatternItem::Rest,
                PatternItem::Note {
                    key: 42,
                    vel: crate::pattern::Velocity::Default
                }
            ],
            parse("m>m   c, r h").as_slice()
        );
    }
}
