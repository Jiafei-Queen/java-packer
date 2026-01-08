# Java Packer
## 项目概述

这是一个基于 Rust 的 Java 打包工具，名为“java-packer”或“jpc”。它提供了一些命令，用于配合 Java 的 jlink 和 jpackage 工具，创建最小化的 JRE 镜像和 Java 应用程序的可分发包。

## 主要命令

该工具支持以下命令：
- `init`：初始化新的配置文件
- `link`：使用 jlink 创建更小的 JRE 镜像
- `package`：使用 jpackage 将 JAR 文件和 JRE 打包成可分发包
- `clean`：删除生成的输出目录

## 配置格式

配置文件使用类似 TOML 的格式，包含以下部分：
- `[LINK]` 部分用于 jlink 选项
- `[PACKAGE]` 部分用于 jpackage 选项

每个部分都包含键值对，其中值是带引号的字符串。 ## 架构

代码库包含以下文件：
- `src/main.rs`：入口点和命令路由
- `src/config_manager.rs`：配置文件解析和初始化
- `src/data.rs`：配置数据结构的类型定义
- `src/executor.rs`：执行 jlink 和 jpackage 命令的核心逻辑
- `src/clean.rs`：清理生成输出目录的功能

## 构建和开发

构建项目：
```bash
cargo build
```

运行测试（如果存在）：
```bash
cargo test
```

运行工具：
```bash
./target/debug/jpc [COMMAND] [OPTIONS]
```

## 需要理解的关键文件

1. `src/main.rs` - 主入口点，处理命令行参数并将命令路由到相应的函数
2. `src/config_manager.rs` - 解析配置文件并处理初始化
3. `src/executor.rs` - 包含执行 jlink 和 jpackage 命令的核心逻辑
4. `src/clean.rs` - 处理生成文件的清理

## 重要注意事项

- 该工具需要系统中安装 Java JDK，并且 jlink 和 jpackage 工具必须在系统 PATH 环境变量中
- 配置文件应采用类似 TOML 的格式，并包含 [SECTION] 标头
- 该工具在执行可能具有破坏性的操作之前会提供交互式确认
- 它在 executor 模块中使用不安全代码来实现全局 QUIET 标志