default:
  just --list

client:
  cd ./crates/client && trunk serve --config ./Trunk.toml

server:
  cd ./crates/server && bun run dev

fmt:
  cargo clippy --fix --workspace --allow-dirty --allow-staged && cargo fmt
  leptosfmt ./crates/client/src/**/*.rs
