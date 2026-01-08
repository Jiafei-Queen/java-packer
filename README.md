# Java Packer

一个用 Rust 编写的命令行工具，用于简化 Java 应用程序的打包流程。它利用 `jlink` 创建精简的 Java 运行时环境 (JRE)，并可选择性地使用 `jpackage` 将应用程序及其 JRE 打包成原生安装包（如 `.exe`, `.deb`, `.pkg` 等），或创建可分发的自包含目录。

## 功能特性

*   **自定义 JRE**: 使用 `jlink` 仅包含应用程序所需的 Java 模块，显著减小最终包的体积。
*   **原生打包**: （可选）使用 `jpackage` 创建平台特定的原生安装包或应用程序。
*   **便携式分发**: （可选）创建一个包含 JRE 和启动脚本的独立目录，可直接运行。
*   **配置驱动**: 通过 `jpc.toml` 配置文件灵活控制打包过程。
*   **跨平台**: 支持在 Windows, Linux, macOS 上运行，并可为目标平台打包。

## 安装

### 从源码构建

1.  确保你已经安装了 [Rust 工具链](https://www.rust-lang.org/tools/install)。
2.  克隆或下载此项目源代码。
3.  在项目根目录下运行：
    ```bash
    cargo build --release
    ```
4.  编译后的可执行文件位于 `target/release/` 目录下（Linux/macOS 上可能是 `jpc`，Windows 上是 `jpc.exe`）。

### 预编译二进制文件
> 请查看本 Repo 的 [Release 页面](https://github.com/Jiafei-Queen/java-packer/releases)

## 快速开始

1.  **初始化配置**: 在你的 Java 项目根目录下运行：
    ```bash
    jpc init
    ```
    这将生成一个 `jpc.toml` 配置文件，你需要根据项目情况进行修改。

2.  **编辑配置文件**: 打开 `jpc.toml`，根据你的项目设置相应的路径和选项（详见下文配置说明）。

3.  **执行打包**:
    *   **创建自包含目录 (Cross-platform)**: 运行 `jpc cross`。这将创建一个包含 JRE、JAR 文件和启动脚本的目录。
    *   **执行 jlink**: 运行 `jpc link`。这将只执行 `jlink` 命令。
    *   **执行 jpackage**: 运行 `jpc package`。这将只执行 `jpackage` 命令（需要先有 `jlink` 生成的 JRE）。
    *   **清理**: 运行 `jpc clean`。这将删除配置文件中指定的输出目录。

## 配置文件 (jpc.toml)

`jpc` 的行为由项目根目录下的 `jpc.toml` 文件控制。`jpc init` 命令会生成一个包含默认值和说明的模板。

### `[LINK]` 部分

配置 `jlink` 命令。

*   `output`: (必需) `jlink` 生成的自定义 JRE 目录路径。
*   `module-path`: (必需) JMods (Java 模块) 的路径。多个路径用 `:` (Linux/macOS) 或 `;` (Windows) 分隔。例如：`"$JAVA_HOME/jmods"` 或 `"$JAVA_HOME/jmods:/path/to/jfx-jmods"`.
*   `add-modules`: (必需) 要添加到 JRE 的模块列表，用逗号分隔。例如：`"java.base,java.desktop"`.
*   `no-header-files`: (可选, bool) 是否排除头文件，默认 `false`。
*   `no-man-pages`: (可选, bool) 是否排除手册页，默认 `false`。
*   `strip-debug`: (可选, bool) 是否移除调试信息，默认 `false`。
*   `compress`: (可选, int) 压缩级别 (0, 1, 2)，默认 `0` (无压缩)。

### `[PACKAGE]` 部分

配置 `jpackage` 命令。

*   `input`: (必需) 包含 JAR 文件和 JRE 的输入目录路径 (通常由 `jlink` 生成)。
*   `name`: (必需) 最终生成的应用程序/安装包名称。
*   `app-version`: (可选) 应用程序版本。
*   `output`: (必需) `jpackage` 输出目录路径。
*   `main-class`: (必需) JAR 文件中包含 `main` 方法的类名。
*   `main-jar`: (必需) 要打包的 JAR 文件名。
*   `type`: (可选) 打包类型 ("app-image", "exe", "msi", "deb", "rpm", "pkg", "dmg")。默认由平台决定。
*   `java-options`: (可选) 传递给 Java 运行时的选项列表。例如：`['-Xmx2G', '-Dfile.encoding=UTF-8']`.
*   `icon`: (可选) 应用程序图标的路径。
*   `verbose`: (可选, bool) 是否启用详细输出，默认 `false`。

### `[CROSS]` 部分

配置 `jpc cross` 命令。

*   `output`: (必需) 生成的自包含目录路径。
*   `jar`: (必需) 你的应用程序 JAR 文件的路径 (相对于当前工作目录)。
*   `jre`: (必需) 用于分发的 JRE 目录路径 (通常由 `jlink` 生成)。

## 使用示例

假设你已经有一个名为 `myapp.jar` 的 Java 应用程序。

1.  运行 `jpc init`。
2.  编辑 `jpc.toml`:
    ```toml
    [LINK]
    output = "build/jre"
    module-path = "$JAVA_HOME/jmods"
    add-modules = "java.base,java.logging,java.xml,java.desktop"

    [PACKAGE]
    input = "build/jre"
    name = "MyApp"
    app-version = "1.0.0"
    output = "build/package"
    main-class = "com.example.Main"
    main-jar = "myapp.jar"
    type = "app-image" # 或 "exe", "deb", "pkg" 等

    [CROSS]
    output = "build/cross"
    jar = "myapp.jar"
    jre = "build/jre"
    ```
3.  执行 `jpc link` 创建 JRE。
4.  执行 `jpc package` 创建原生安装包。
5.  或者执行 `jpc cross` 创建便携式分发目录。

## 命令行选项

*   `jpc init`: 生成 `jpc.toml` 配置文件模板。
*   `jpc link`: 执行 `jlink` 创建自定义 JRE。
*   `jpc package`: 执行 `jpackage` 创建原生安装包或应用镜像。
*   `jpc cross`: 创建包含 JRE 和 JAR 的可分发目录及启动脚本。
*   `jpc clean`: 删除配置文件中指定的目录。
*   `jpc -h, --help`: 显示帮助信息。
*   `jpc -v, --version`: 显示工具版本。
*   `jpc -q, --quiet`: 静默模式，减少输出信息。
*   `jpc -c <FILE>, --config <FILE>`: 指定自定义的配置文件路径 (默认为 `jpc.toml`)。

## 注意事项

*   确保你的系统已安装 JDK 14 或更高版本，并且 `jlink` 和 `jpackage` 可执行文件在系统 `PATH` 中。
*   `jpackage` 的功能和可用选项在不同 JDK 版本间可能有所差异。
*   在 Windows 上，使用 `jpackage` 创建 `.exe` 或 `.msi` 包可能需要额外的工具（如 WiX Toolset）。
*   在 macOS 上，使用 `jpackage` 创建 `.pkg` 或 `.dmg` 包可能需要开发者证书。
*   `jpc cross` 生成的 `run.sh` 脚本在 Linux/macOS 上会自动设置执行权限。

## 许可证: MIT
[许可证](LICENSE)