build:
    cargo build


[no-cd]
test:
    pwd
    cargo test

test-all:
    bash scripts/test-all.sh

