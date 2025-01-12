alias w := watch

watch +WATCH_TARGET='run':
    watchexec -rc -w . --ignore *.results -- just {{WATCH_TARGET}}

run:
    cargo run -- --bpm 180 --out target/4_4.mid 
    cargo run -- --bpm 180 --subs 2 --out target/clave23.mid --pattern "rrcrcrrrcrrcrrcr"
    cargo run -- --bpm 180 --subs 2 --out target/cascara.mid  --pattern "h>r h>r hh> rh h>r hh> rh> rh"
    cargo run -- --bpm 180 --subs 2 --out target/clave32.mid --pattern "crrcrrcrrrcrcrrr"
    cargo run -- --bpm 180 --subs 3 --out target/triplet.mid  --pattern "c>cc ccc ccc> cc>c"

tracks: build-release
    #!/usr/bin/env bash
    set -euxo pipefail
    mkdir -p tracks
    mkdir -p tracks/44
    mkdir -p tracks/44_2
    mkdir -p tracks/44_3
    mkdir -p tracks/clave23
    mkdir -p tracks/clave32

    for ((i=40; i<=300; i+=5)); do
        ./target/release/clave --bpm $i --beats 10000 --pattern "m>mmm" --out "tracks/44/44_$i.mid"
        ./target/release/clave --bpm $i --beats 10000 --subs 2 --pattern "m>m,mm,mm,mm," --out "tracks/44_2/44_2_$i.mid"
        ./target/release/clave --bpm $i --beats 10000 --subs 3 --pattern "m>m,m,mm,m,mm,m,mm,m," --out "tracks/44_3/44_3_$i.mid"
        ./target/release/clave --bpm $i --beats 10000 --subs 2 --pattern "rrcrcrrrcrrcrrcr" --out "tracks/clave23/clave23_$i.mid"
        ./target/release/clave --bpm $i --beats 10000 --subs 2 --pattern "crrcrrcrrrcrcrrr" --out "tracks/clave32/clave32_$i.mid"
    done

build-release:
    cargo build -r