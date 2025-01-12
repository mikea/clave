# Clave

Clave is a midi click-track generator. 
It generates single-track single-channel midi file
with a click pattern.

Note: clave tracks are percussion by its nature. 
Because of this clave does not generate note off events
(only note on).

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
    - `m` - metronome click
    - `c` - clave click
    - `h` - closed hi-hat
    - `r` - rest
- and optional volume modifier:
    - `>` - accented note
    - `,` - ghost note

Default pattern is `m>mmm` which corresponds to 4/4
metronome click with accented first beat.

Some usefull patterns are:

| Description | Arguments |
|---|---|
|4/4 with ghost 8th|`--subs 2 -p "m>m, mm, mm, mm,"`
|Son clave 3-2|`--subs 2 -p "cr rc rr cr  rr cr cr rr"`
|Son clave 2-3|`--subs 2 -p "rr cr cr rr  cr rc rr cr"`
|Rumba clave 3-2|`--subs 2 -p "cr rc rr rc  rr cr cr rr"`
|Rumba clave 2-3|`--subs 2 -p "rr cr cr rr  cr rc rr rc"`
|Cascara 2-3|`--subs 2 -p "h>r h>r hh> rh h>r hh> rh> rh"`
