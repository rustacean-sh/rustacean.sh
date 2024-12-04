<h1 align=center><code>rustacean.sh</code></h1>
<p align=center>Rustaceans Network</p>

## Development

### Running Locally

> [!IMPORTANT]
> Make sure you have gone through the [Prerequisites](#prerequisites) section before
> running the following commands.

This project uses a Client ↔ Server architecture, two process (one for each of these components)
should be runned in order to run the application locally.

Create a terminal session for the client, and run the following command:

```bash
just client
```

Then create another terminal session for the server, and run the following command:

```bash
just server
```

### Prerequisites

1. [Rust](https://rustup.rs)
2. [Bun](https://bun.sh)
3. [Justfile](https://github.com/casey/just) (**optional**)
4. The `wasm32-unknown-unknown` toolchain
5. `worker-build` binary
6. `trunk` binary

### Installation Guides

#### Rust

Rust can be installed by running the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then install the `wasm32-unknown-unknown` toolchain by running:

```bash
rustup target add wasm32-unknown-unknown
```

#### Bun

Bun can be installed by running the following command in your terminal:

```bash
curl -fsSL https://bun.sh/install | bash
```

#### Rust Binaries

Justfile can be installed by running the following command in your terminal:

```bash
cargo install just
```

Worker Build can be installed by running the following command in your terminal:

```bash
cargo install worker-build
```

Trunk can be installed by running the following command in your terminal:

```bash
cargo install trunk
```

> Read more here: https://developers.cloudflare.com/workers/languages/rust/
