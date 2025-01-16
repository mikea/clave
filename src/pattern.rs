use nom::branch::alt;
use nom::character::complete::{char, digit1, multispace0};
use nom::combinator::{map, map_res, success};
use nom::multi::many1;
use nom::sequence::{delimited, pair, terminated};
use nom::{IResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PatternItem {
    Rest,
    Notes(Vec<Note>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub key: u8,
    pub vel: Velocity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Velocity {
    Default,
    Accented,
    Ghost,
}

pub(crate) fn parse(s: &str) -> Result<Vec<PatternItem>, String> {
    let (rest, result) =
        pattern(s).map_err(|err| format!("error parsing pattern `{s}`: {err:?}"))?;
    if !rest.is_empty() {
        return Err(format!("invalid pattern, unexpected `{rest}`"));
    }
    Ok(result)
}

fn pattern(i: &str) -> IResult<&str, Vec<PatternItem>> {
    many1(item).parse(i)
}

fn item(i: &str) -> IResult<&str, PatternItem> {
    alt((
        rest,
        map(note, |n| PatternItem::Notes(vec![n])),
        map(chord, PatternItem::Notes),
    )).parse(i)
}

fn rest(i: &str) -> IResult<&str, PatternItem> {
    terminated(map(char('r'), |_| PatternItem::Rest), multispace0).parse(i)
}

fn key(i: &str) -> IResult<&str, u8> {
    alt((
        map(char('m'), |_| 32),
        map(char('c'), |_| 75),
        map(char('h'), |_| 42),
        map(char('b'), |_| 35),
        map(char('s'), |_| 38),
        map_res(digit1, |x: &str| x.parse()),
    )).parse(i)
}

fn velocity(i: &str) -> IResult<&str, Velocity> {
    alt((
        map(char('>'), |_| Velocity::Accented),
        map(char(','), |_| Velocity::Ghost),
        success(Velocity::Default),
    )).parse(i)
}

fn note(i: &str) -> IResult<&str, Note> {
    terminated(
        map(pair(key, velocity), |(key, vel)| Note { key, vel }),
        multispace0,
    ).parse(i)
}

fn chord(i: &str) -> IResult<&str, Vec<Note>> {
    delimited(
        terminated(char('('), multispace0),
        many1(note),
        terminated(char(')'), multispace0),
    ).parse(i)
}

#[cfg(test)]
mod test {

    use crate::pattern::{parse, Note, PatternItem, Velocity};

    #[test]
    fn rest() {
        assert_eq!(super::rest("r").unwrap(), ("", PatternItem::Rest));
    }

    #[test]
    fn key() {
        assert_eq!(super::key("m").unwrap(), ("", 32));
        assert_eq!(super::key("h").unwrap(), ("", 42));
        assert_eq!(super::key("c").unwrap(), ("", 75));
        assert_eq!(super::key("b").unwrap(), ("", 35));
        assert_eq!(super::key("s").unwrap(), ("", 38));

        assert_eq!(super::key("2").unwrap(), ("", 2));
        assert_eq!(super::key("32").unwrap(), ("", 32));
        assert_eq!(super::key("102").unwrap(), ("", 102));
    }

    #[test]
    fn velocity() {
        assert_eq!(super::velocity("").unwrap(), ("", Velocity::Default));
        assert_eq!(super::velocity(",").unwrap(), ("", Velocity::Ghost));
        assert_eq!(super::velocity(">").unwrap(), ("", Velocity::Accented));
    }

    #[test]
    fn note() {
        assert_eq!(
            super::note("c>").unwrap(),
            (
                "",
                Note {
                    key: 75,
                    vel: Velocity::Accented
                }
            )
        );
        assert_eq!(
            super::note("h,").unwrap(),
            (
                "",
                Note {
                    key: 42,
                    vel: Velocity::Ghost
                }
            )
        );
        assert_eq!(
            super::note("m").unwrap(),
            (
                "",
                Note {
                    key: 32,
                    vel: Velocity::Default
                }
            )
        );
    }

    #[test]
    fn bad_pattern() {
        assert_eq!(
            Err(
                "error parsing pattern `???`: Error(Error { input: \"???\", code: Char })"
                    .to_string()
            ),
            parse("???")
        );
        assert_eq!(
            Err("invalid pattern, unexpected `.`".to_string()),
            parse("r.")
        );
    }

    #[test]
    fn parse_pattern() {
        assert_eq!(
            &[PatternItem::Rest, PatternItem::Rest, PatternItem::Rest],
            parse("rrr").unwrap().as_slice()
        );
        assert_eq!(
            &[
                PatternItem::Notes(vec![Note {
                    key: 32,
                    vel: Velocity::Accented
                }]),
                PatternItem::Notes(vec![Note {
                    key: 32,
                    vel: Velocity::Default
                }]),
                PatternItem::Notes(vec![Note {
                    key: 75,
                    vel: Velocity::Ghost
                }]),
                PatternItem::Rest,
                PatternItem::Notes(vec![Note {
                    key: 42,
                    vel: Velocity::Default
                }])
            ],
            parse("m>mc,rh").unwrap().as_slice()
        );

        assert_eq!(
            &[
                PatternItem::Notes(vec![Note {
                    key: 32,
                    vel: Velocity::Accented
                }]),
                PatternItem::Notes(vec![Note {
                    key: 32,
                    vel: Velocity::Default
                }]),
                PatternItem::Notes(vec![Note {
                    key: 75,
                    vel: Velocity::Ghost
                }]),
                PatternItem::Rest,
                PatternItem::Notes(vec![Note {
                    key: 42,
                    vel: Velocity::Default
                }])
            ],
            parse("m>m   c, r h").unwrap().as_slice()
        );
        assert_eq!(
            &[
                PatternItem::Notes(vec![
                    Note {
                        key: 75,
                        vel: Velocity::Default
                    },
                    Note {
                        key: 42,
                        vel: Velocity::Default
                    }
                ]),
                PatternItem::Notes(vec![
                    Note {
                        key: 75,
                        vel: Velocity::Accented
                    },
                    Note {
                        key: 42,
                        vel: Velocity::Ghost
                    }
                ]),
                PatternItem::Notes(vec![Note {
                    key: 75,
                    vel: Velocity::Default
                }]),
                PatternItem::Notes(vec![Note {
                    key: 42,
                    vel: Velocity::Default
                }]),
            ],
            parse("(ch)(c>h,)ch").unwrap().as_slice()
        );
    }
}
