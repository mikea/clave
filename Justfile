alias w := watch

watch +WATCH_TARGET='run':
    watchexec -rc -w . --ignore *.results -- just {{WATCH_TARGET}}

run:
    cargo run -- --bpm 125 --out target/44.mid 
    cargo run -- --bpm 125 --subs 2 --out target/clave23.mid --pattern "rrcrcrrrcrrcrrcr"
    cargo run -- --bpm 125 --subs 2 --out target/cascara.mid  --pattern "c>rc>rcc>rcc>rcc>rcrc"
    cargo run -- --bpm 125 --subs 2 --out target/clave32.mid --pattern "crrcrrcrrrcrcrrr"
    cargo run -- --bpm 125 --subs 3 --out target/triplet.mid  --pattern "c>cc ccc ccc> cc>c"
