# AGENTS.md

本指南适用于所有针对 playground-rust 的 AI 辅助贡献，其中介绍了该项目的架构、命令及开发工作流.

## 项目概述

这是学习`playground-rust`的项目, 主要用于学习Rust语言. 具体每个示例都在独立的目录中.

## 项目架构

## 技术栈

- 语言: Rust (Edition 2024, MSRV 1.94)
- 编译系统: Cargo

## 开发流程

在工作空间中执行以下命令, 如果已进入到示例目录不需要指定 `-p <project-name>`.

### 编译与运行

```sh
# Development build 
cargo build -p <project-name>

# Release build (optimized)
cargo build -p <project-name> --release 

# Run directly
cargo run -p <project-name> -- <command>

# Run example
cargo run -p <project-name> --example <example-name>

# Install locally
cargo install -p <project-name> --path .
```

### 测试

```sh
# Run all tests
cargo test -p <project-name>

# Run all tests all features
cargo test -p <project-name> --all-features

# Run specific test
cargo test -p <project-name> <test_name>

# Run tests with output
cargo test -p <project-name> -- --nocapture

# Run tests in specific module
cargo test -p <project-name> <module_name>::
```

### 代码检查与质量

```sh
# Check without building
cargo check -p <project-name>

# Format code
cargo fmt -p <project-name> -- --check

# Run clippy lints
cargo clippy -p <project-name>

# Check all targets and features
cargo clippy -p <project-name> --all-targets --all-features -- -D warnings
```

## 错误处理

遵循 Rust 错误处理的最佳实践：

规则：

- 使用**anyhow::Result**（一个应用程序，而非库）
- **始终使用** `?` 运算符，并“建议”配合 `?` 运算符使用 `.context("description")`
- **禁止使用unwrap()** （仅限测试——如有需要，请使用 `expect("explanation")`）

## 边界条件

**don't panic on failure**（会中断用户的工作流）。始终使用 `?` 运算符，如有需要，请将日志输出到标准输出。
