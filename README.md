# rwatch

A cross-platform, high-performance command monitoring utility written in Rust. 

## Description

rwatch is a lightweight alternative to the Unix 'watch' command. It executes a specified command at regular intervals and displays the output in the terminal, clearing the screen between each execution. It is designed to work natively on both Linux (Fedora/Kali/Ubuntu) and Windows.

## Features

* Cross-platform compatibility (Windows and Linux).
* Configurable execution intervals.
* Low CPU usage via thread sleeping.
* Displays both standard output (stdout) and standard error (stderr).
* ANSI escape sequences for screen clearing.

## Requirements

* Rust / Cargo (Latest stable version recommended)
* Linux: /bin/sh (Standard on Fedora)
* Windows: cmd.exe

## Installation

1. Clone the repository:
   git clone <your-repository-url>
   cd rwatch

2. Build the release binary:
   cargo build --release

3. (Optional) Move to path for system-wide use:
   sudo mv target/release/rwatch /usr/local/bin/

## Usage

Run the executable with the interval in seconds and the command in quotes.

Syntax:
rwatch <seconds> "<command>"

Example:
rwatch 2 "ls -la"
rwatch 5 "netstat -ant"

## How it Works

The tool uses conditional compilation (cfg macros) to select the appropriate shell:
- Linux: Uses 'sh -c'
- Windows: Uses 'cmd /C'

It utilizes 'std::thread::sleep' to ensure the process does not consume unnecessary CPU cycles during the wait interval.
