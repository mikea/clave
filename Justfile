alias w := watch

watch +WATCH_TARGET='run':
    watchexec -rc -w . --ignore *.results -- just {{WATCH_TARGET}}

run:
    cargo run -- --bpm 300 --out target/clave23.mid --pattern "rrcrcrrrcrrcrrcr"
    cargo run -- --bpm 300 --out target/cascara.mid  --pattern "c>rc>rcc>rcc>rcc>rcrc"
    cargo run -- --bpm 300 --out target/clave32.mid --pattern "crrcrrcrrrcrcrrr"
