![Crates.io Version](https://img.shields.io/crates/v/clave)

# Clave

Clave is a midi click-track generator. 
It generates single-track single-channel midi file
with a customizable click pattern.

Note: click tracks are percussion in their nature. 
Because of this clave does not generate note off events
(only note on).

## Pregenerated Tracks

A lot of pregenerated tracks are available
in [tracks](https://github.com/mikea/clave/tree/master/tracks) folder.

## Installation

`cargo install clave`

## Usage

By default Clave generates 1000 beats of 120bpm 4/4 clicks:

```
clave --out 44_120.mid
```

You can use command line arguments to customize the track:

```
  -b, --bpm <BPM>              Track tempo (beats per minute) [default: 120]
      --channel <CHANNEL>      Midi channel [default: 10]
      --beats <BEATS>          Track length in number of beats [default: 1000]
      --vel <VEL>              Default note velocity [default: 63]
      --acc-vel <ACC_VEL>      Accented note velocity [default: 94]
      --ghost-vel <GHOST_VEL>  Ghost note velocity [default: 31]
      --subs <SUBS>            Number of subdivisions in a pattern [default: 1]
  -p, --pattern <PATTERN>      Click pattern [default: m>mmm]
```

### Click Pattern

You can use `--pattern` together with `--subs` to create
more sophisticated click patterns:

- the pattern consists of pattern items optionally separated by spaces
- each pattern item consists of the note:
    - `m` - metronome click (32)
    - `c` - clave click (75)
    - `b` - bass drum (35)
    - `s` - snare (38)
    - `h` - closed hi-hat (42)
    - `XX` or `XXX` - decimal number of the percussion key (https://en.wikipedia.org/wiki/General_MIDI#Percussion)
    - `r` - rest
- and optional volume modifier:
    - `>` - accented note
    - `,` - ghost note
- multiple notes can be played as a chord simultaneosly by surrounding them with `(` `)`.

Default pattern is `m>mmm` which corresponds to 4/4
metronome click with accented first beat.

### Pattern Examples

| Description | Subs | Pattern
|---|---|--|
|4/4 with ghost 8th| `2` | `"m>m,mm,mm,mm,"`
|Son clave 3-2| `2` | `"crrcrrcrrrcrcrrr"`
|Son clave 2-3|  `2` | `"rrcrcrrrcrrcrrcr"`
|Rumba clave 3-2| `2` | `"crrcrrrcrrcrcrrr"`
|Rumba clave 2-3| `2` | `"rrcrcrrrcrrcrrrc"`
|Cascara 2-3| `2` | `"h>rh>rhh>rhh>rhh>rh>rh"`
|Rock 4/4| `2` | `"(bh)h(sh)h(bh)h(sh)h"`
|Funk 4/4| `4` | `"(bh)h>hb s>rhh> bhh>b (s>h>)r(s>h>)r"`
|Funk with Ghost Notes| `4` | `"(bh)s,hs> hs,hs, hs,(bh)r (s>h)rhr"`
