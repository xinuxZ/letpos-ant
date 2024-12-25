# Ant Leptos

[English](./README_en.md) | 简体中文

> 🦀 基于 Leptos 的 Rust UI 组件库，设计风格遵循 Ant Design 规范

[![Crates.io](https://img.shields.io/crates/v/ant-leptos.svg)](https://crates.io/crates/ant-leptos)
[![Documentation](https://docs.rs/ant-leptos/badge.svg)](https://docs.rs/ant-leptos)
[![License](https://img.shields.io/crates/l/ant-leptos.svg)](LICENSE)
[![Rust](https://github.com/ant-leptos/ant-leptos/workflows/CI/badge.svg)](https://github.com/ant-leptos/ant-leptos/actions)

Ant Leptos 是一个基于 [Leptos](https://leptos.dev/) 框架的 Rust UI 组件库，提供了一套完整的企业级 UI 组件，设计风格遵循 [Ant Design](https://ant.design/) 规范。

[在线演示](https://ant-leptos.github.io) | [组件文档](https://ant-leptos.github.io/docs) | [更新日志](./CHANGELOG.md) | [贡献指南](./CONTRIBUTING.md)

## ✨ 特性

- 🦀 使用 Rust 语言开发
- 🔥 基于 Leptos 0.7.2 框架
- 🎨 遵循 Ant Design 设计规范
- 🌍 支持国际化
- 🎭 支持主题切换
- 📦 组件按需加载
- 🚀 WebAssembly 支持

## 📦 安装

将以下依赖添加到你的 `Cargo.toml` 文件中：

```toml
[dependencies]
ant-leptos = "0.1.0"
leptos = { version = "0.7.2", features = ["csr"] }
```

## 🔨 示例

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

## 🔨 本地开发

### 环境要求

- Rust 1.75.0 或更高版本
- wasm-pack
- trunk
- cargo-leptos

### 开发步骤

1. 克隆仓库：
```bash
git clone https://github.com/your-username/ant-leptos.git
cd ant-leptos
```

2. 安装依赖：
```bash
cargo install trunk wasm-pack cargo-leptos
```

3. 运行示例：
```bash
cd examples/basic
trunk serve
```

然后在浏览器中访问 `http://localhost:8080`。

## 📦 目录结构

```
ant-leptos/
├── src/
│   ├── components/     # 组件实现
│   ├── styles/        # 组件样式
│   └── utils/         # 工具函数
├── examples/          # 示例代码
├── tests/            # 测试文件
└── docs/            # 文档
```

## 🔨 组件列表

目前已实现的组件：

### 核心基础
- ✅ ConfigProvider - 全局配置
- ✅ Theme - 主题配置
- ✅ Style - 全局样式
- ✅ Locale - 国际化
- ✅ Version - 版本信息

### 基础组件
- 🚧 Typography - 排版
- 🚧 Space - 间距
- 🚧 Divider - 分割线
- 🚧 Grid - 栅格
- 🚧 Layout - 布局
- 🚧 Icon - 图标
- 🚧 Button - 按钮

更多组件正在开发中...

## 🤝 贡献指南

我们欢迎任何形式的贡献，包括但不限于：

- 提交问题和建议
- 改进文档
- 修复 bug
- 添加新功能
- 改进代码质量

请确保在提交 PR 之前：

1. 运行测试：`cargo test`
2. 运行 lint：`cargo clippy`
3. 格式化代码：`cargo fmt`

## 📝 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件
