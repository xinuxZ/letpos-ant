# Ant Leptos

English | [简体中文](./README.md)

Ant Leptos is a Rust UI component library based on the [Leptos](https://leptos.dev/) framework, providing a complete set of enterprise-level UI components following [Ant Design](https://ant.design/) specifications.

## ✨ Features

- 🦀 Developed with Rust
- 🔥 Based on Leptos 0.7.2 framework
- 🎨 Follows Ant Design specifications
- 🌍 Internationalization support
- 🎭 Theme switching support
- 📦 On-demand component loading
- 🚀 WebAssembly support

## 📦 Installation

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
ant-leptos = "0.1.0"
leptos = { version = "0.7.2", features = ["csr"] }
```

## 🔨 Example

```rust
use ant_leptos::components::{Button, ConfigProvider};
use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Button>"Hello Ant Leptos!"</Button>
        </ConfigProvider>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
```

## 🔨 Local Development

### Requirements

- Rust 1.75.0 or higher
- wasm-pack
- trunk
- cargo-leptos

### Development Steps

1. Clone repository:
```bash
git clone https://github.com/your-username/ant-leptos.git
cd ant-leptos
```

2. Install dependencies:
```bash
cargo install trunk wasm-pack cargo-leptos
```

3. Run example:
```bash
cd examples/basic
trunk serve
```

Then visit `http://localhost:8080` in your browser.

## 📦 Directory Structure

```
ant-leptos/
├── src/
│   ├── components/     # Component implementations
│   ├── styles/        # Component styles
│   └── utils/         # Utility functions
├── examples/          # Example code
├── tests/            # Test files
└── docs/            # Documentation
```

## 🔨 Component List

Currently implemented components:

### Core Foundation
- ✅ ConfigProvider - Global configuration
- ✅ Theme - Theme configuration
- ✅ Style - Global styles
- ✅ Locale - Internationalization
- ✅ Version - Version information

### Basic Components
- 🚧 Typography
- 🚧 Space
- 🚧 Divider
- 🚧 Grid
- 🚧 Layout
- 🚧 Icon
- 🚧 Button

More components are under development...

## 🤝 Contributing

We welcome all forms of contributions, including but not limited to:

- Submitting issues and suggestions
- Improving documentation
- Fixing bugs
- Adding new features
- Improving code quality

Before submitting a PR, please ensure:

1. Run tests: `cargo test`
2. Run lint: `cargo clippy`
3. Format code: `cargo fmt`

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
