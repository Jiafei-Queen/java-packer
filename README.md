# Java Packer (jpc)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust 2024](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)


由 Rust 编写的 Java 应用打包工具。

## 适合做什么

1. 快速为 Java 程序构建绿色目录（cross 功能）
2. 方便调用 jlink/jpackage 命令裁剪运行时，打包应用

## 快速开始

### 生成配置文件
```bash
jpc init        # 生成模板配置文件
```

### 编辑配置文件
按照项目实际修改配置文件

### 执行命令

**1. 利用 jlink/jpackage**
```bash
jpc link        # 根据配置文件生成裁剪后的 JRE
jpc package     # 根据配置文件打包 bundle
```

**2. 快速生成 Java 绿色目录（不需要 jlink/jpackage，快速生成）**
```bash
jpc cross-unix  # for Linux, macOS, FreeBSD...
jpc cross-win   # for Windows
```

## 为什么选择 jpc？

- 不依赖 Maven/Gradle 构建工具
- 配置文件跟随项目
- 方便，快速

## 依赖

### 对于 link/package 功能

（调用 jlink, jpackage 生成简化 JRE 和 应用 bundle）

- 已安装 JDK 14+，并确保 `jlink` / `jpackage` 在 `PATH` 中。
- Windows 打包 `.msi` 仍需要 WiX Toolset（`jpackage` 的要求）。
- macOS 打包 `.pkg` / `.dmg` 可能需要开发者证书（`jpackage` 的要求）。

### 对于 cross 功能

（生成绿色 Java 应用目录）

- 至少需要在配置文件中配置一个 JRE 目录

## 安装

### 从源码构建

**需求：**
- 基础 Rust 编译环境
- `x86_64-pc-windows-gnu` 目标支持
- Windows 用户需要 Bash 环境（自己复制脚本内容执行也可以）

```bash
./mach    # 构建脚本
```

产物在 `target/release/`（Linux/macOS 为 `jpc`，Windows 为 `jpc.exe`）。

### 预编译二进制

Release 页面：<https://github.com/Jiafei-Queen/java-packer/releases>


## 配置文件说明（jpc.ini）

配置文件是一个简单的 `key = "value"` 格式，所有值必须用双引号包起来。支持变量替换：在 `[VAR]` 中定义变量，然后在其他 section 用 `$VAR$` 引用。

### `[VAR]`

可选，用于定义变量并替换到其他 section 的值里。

### `[LINK]`（jlink）

该 section 的每个 `key` 会变成 `--key "value"` 传给 `jlink`。没有键值的开关请放在 `default-arg` 中（空格分隔）。

常用键：
- `module-path`：多个路径用系统分隔符连接（Unix `:`，Windows `;`）
- `add-modules`：逗号分隔的模块列表
- `output`：输出目录
- `compress`：0/1/2 或者在 JDK21 后的：zip-0...9
- `default-arg`：例如 `--no-header-files --no-man-pages --strip-debug`

### `[PACKAGE]`（jpackage）

该 section 的每个 `key` 会变成 `--key "value"` 传给 `jpackage`。

常用键示例：
- `name`
- `type`
- `main-jar`
- `main-class`
- `app-version`
- `runtime-image`
- `input`
- `dest`

### `[CROSS]`（快速分发）

生成一个目录结构：
```
output/
  |_ runtime/   # 复制自 runtime-image
  |- target/    # 复制自 input/main-jar
  |- output-exec
```

必填键：
- `output`
- `input`
- `main-jar`
- `runtime-image`
- `output-exec`
