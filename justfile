preview:
    quarto preview --port 3333

check:
    cargo check --examples --manifest-path intro-to-rust/Cargo.toml

render:
    quarto render

update:
    git add . && git commit -m "update" && git push
