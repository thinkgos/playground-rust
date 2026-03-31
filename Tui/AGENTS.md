# AGENTS.md

本指南适用于所有针对 Tui 的 AI 辅助贡献，其中介绍了该项目的架构、命令及开发工作流.

## 项目概述

这是学习`tui`的项目, 基于`ratatui`,`crossterm`的终端应用, 每一个示例都是一个独立的终端应用.

## 项目架构

## 技术栈

- 语言: Rust (Edition 2024, MSRV 1.94)
- 编译系统: Cargo

## 依赖

- `ratatui`: 用于终端应用的UI库
- `crossterm`: 跨平台终端输入输出库
- `anyhow`: 错误处理库.

## 开发流程

进入相应的示例目录, 并执行以下命令:

### 编译与运行

```sh
# Development build 
cargo build

# Release build (optimized)
cargo build --release

# Run directly
cargo run -- <command>

# Run example
cargo run --example <example-name>

# Install locally
cargo install --path .
```

### 测试

```sh
# Run all tests
cargo test

# Run all tests all features
cargo test --all-features

# Run specific test
cargo test <test_name>

# Run tests with output
cargo test -- --nocapture

# Run tests in specific module
cargo test <module_name>::
```

### 代码检查与质量

```sh
# Check without building
cargo check --all

# Format code
cargo fmt --all -- --check

# Run clippy lints
cargo clippy

# Check all targets and features
cargo clippy --all-targets --all-features -- -D warnings
```

## 错误处理

遵循 Rust 错误处理的最佳实践：

规则：

- 使用**anyhow::Result**（一个应用程序，而非库）
- **始终使用** `?` 运算符，并“建议”配合 `?` 运算符使用 `.context("description")`
- **禁止使用unwrap()** （仅限测试——如有需要，请使用 `expect("explanation")`）

## 边界条件

**on't panic on failure**（会中断用户的工作流）。始终使用 `?` 运算符，如有需要，请将日志输出到标准输出。
