alias w := watch

watch +WATCH_TARGET='run':
    watchexec -rc -w . --ignore *.results -- just {{WATCH_TARGET}}

run:
    cargo run -- --out target/out.midi
