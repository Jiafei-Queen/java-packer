# Java Packer (jpc)

一个用 Rust 写的小工具，负责把 `jlink` / `jpackage` 的常用参数从 `jpc.toml` 读取出来，并按需执行。它不实现新的打包逻辑，只是把你的配置转换成命令行参数，并提供一个快速生成“可运行目录”的 `cross-*` 命令。

## 适合做什么

- 精简 JRE：把 Java 模块按需裁剪后输出到指定目录。
- 原生打包：把 JAR + JRE 交给 `jpackage` 生成安装包或 app-image。
- 快速分发：生成包含 `runtime/`、`target/` 和启动脚本的目录，方便拷贝运行。

## 依赖

- 已安装 JDK 14+，并确保 `jlink` / `jpackage` 在 `PATH` 中。
- Windows 打包 `.msi` 仍需要 WiX Toolset（`jpackage` 的要求）。
- macOS 打包 `.pkg` / `.dmg` 可能需要开发者证书（`jpackage` 的要求）。

## 安装

### 从源码构建

```bash
cargo build --release
```

产物在 `target/release/`（Linux/macOS 为 `jpc`，Windows 为 `jpc.exe`）。

### 预编译二进制

Release 页面：<https://github.com/Jiafei-Queen/java-packer/releases>

## 快速开始

1) 生成配置
```bash
jpc init
```

2) 编辑 `jpc.toml`

3) 执行
```bash
jpc link
jpc package
```

或生成跨平台可运行目录：
```bash
jpc cross-unix
jpc cross-win
```

## 命令列表

- `jpc init` 生成 `jpc.toml` 模板（如果存在会询问是否覆盖）
- `jpc link` 执行 `jlink`
- `jpc package` 执行 `jpackage`
- `jpc cross-unix` 生成 Unix 启动脚本 `run.sh`
- `jpc cross-win` 生成 Windows 启动脚本 `run.bat`
- `jpc clean` 删除配置中定义的输出目录
- `jpc -c <FILE>, --config <FILE>` 指定配置路径（默认 `jpc.toml`）
- `jpc -q, --quiet` 跳过执行前的确认提示
- `jpc -h, --help` / `jpc -v, --version`

## 配置文件说明（jpc.toml）

配置文件是一个简单的 `key = "value"` 格式，所有值必须用双引号包起来。支持变量替换：在 `[VAR]` 中定义变量，然后在其他 section 用 `$NAME$` 引用。

### `[VAR]`

可选，用于定义变量并替换到其他 section 的值里。

### `[LINK]`（jlink）

该 section 的每个 `key` 会变成 `--key "value"` 传给 `jlink`。没有值的开关请放在 `default-arg` 中（空格分隔）。

常用键：
- `module-path`：多个路径用系统分隔符连接（Unix `:`，Windows `;`）
- `add-modules`：逗号分隔的模块列表
- `output`：输出目录
- `compress`：0/1/2
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
<output>/
  runtime/   # 复制自 runtime-image
  target/    # 复制自 input/main-jar
  run.sh 或 run.bat
```

必填键：
- `output`
- `input`
- `main-jar`
- `runtime-image`

## 配置示例

```toml
[VAR]
JAVA_HOME="/Library/Java/JavaVirtualMachines/temurin-21-aarch64.jdk/Contents/Home"
JAR="example-1.0.0.jar"

[LINK]
default-arg="--no-header-files --no-man-pages --strip-debug"
module-path="$JAVA_HOME$/jmods"
add-modules="java.base,java.desktop"
compress="2"
output="runtime"

[PACKAGE]
name="example"
type="app-image"
main-jar="$JAR$"
main-class="com.example.demo.Main"
app-version="1.0.0"
runtime-image="runtime"
input="target"
dest="dist"
vendor="example.com"

[CROSS]
output="example-0.1.0-unix"
input="target"
main-jar="example-0.1.0.jar"
runtime-image="$JAVA_HOME$"
```

## 许可证

MIT，见 `LICENSE`。
