# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based Java packaging tool called "java-packer" or "jpc". It provides commands to work with Java's jlink and jpackage tools to create minimal JRE images and distributable packages for Java applications.

## Main Commands

The tool supports the following commands:
- `init`: Initialize a new configuration file
- `link`: Use jlink to create a smaller JRE image
- `package`: Use jpackage to package a JAR file with JRE into a distributable package
- `clean`: Remove generated output directories

## Configuration Format

Configuration files use a TOML-like format with sections:
- `[LINK]` section for jlink options
- `[PACKAGE]` section for jpackage options

Each section contains key-value pairs where values are quoted strings.

## Architecture

The codebase consists of:
- `src/main.rs`: Entry point and command routing
- `src/config_manager.rs`: Configuration file parsing and initialization
- `src/data.rs`: Type definitions for configuration data structures
- `src/executor.rs`: Core logic for executing jlink and jpackage commands
- `src/clean.rs`: Cleanup functionality for generated output directories

## Build and Development

To build the project:
```bash
cargo build
```

To run tests (if any):
```bash
cargo test
```

To run the tool:
```bash
./target/debug/jpc [COMMAND] [OPTIONS]
```

## Key Files to Understand

1. `src/main.rs` - Main entry point that handles command line arguments and routes to appropriate functions
2. `src/config_manager.rs` - Parses configuration files and handles initialization
3. `src/executor.rs` - Contains the core logic for executing jlink and jpackage commands
4. `src/clean.rs` - Handles cleanup of generated files

## Important Notes

- The tool requires Java JDK with jlink and jpackage utilities to be available in the system PATH
- Configuration files are expected to be in TOML-like format with [SECTION] headers
- The tool provides interactive confirmation before executing potentially destructive operations
- It uses unsafe code for global QUIET flag in executor module