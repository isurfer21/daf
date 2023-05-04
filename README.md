# daf
Directories &amp; Files

**Directories & Files** a.k.a _*daf*_, is a command-line interface (CLI) app developed by [Abhishek Kumar](mailto:akbittu@gmail.com). 

This app allows users to list the directories and files in a specified path with or without attributes as opted.

With `daf`, users can easily view and manage the contents of their directories and files. The app provides several options to customize the output format, including columns, JSON, list, table, and YAML. This makes it easy for users to view the information in a format that best suits their needs.

`daf` is a powerful tool for managing directories and files on your computer. Whether you need to quickly view the contents of a directory or manage your files more efficiently, `daf` has got you covered.

## Installation

This guide provides several methods for installing the _daf_ tool on your computer. Follow the steps below to set up _daf_ using the method that works best for you.

### Method 1: Manual installation

1. Visit the _daf_ GitHub page and navigate to the "Releases" section.
2. Download the latest release for your operating system and extract the contents of the archive.
3. Copy the extracted binary file to a directory in your `$PATH`. On macOS or Linux, this could be `/usr/local/bin`, while on Windows, you could copy the binary to `C:\Windows\system32`.
4. Verify that _daf_ is installed correctly by opening a terminal or command prompt and running the command `daf --version`.

### Method 2: Using Cargo

If you have a Rust development environment set up on your computer, you can use the `cargo install` command to install _daf_:

1. Open a terminal or command prompt and run the command `cargo install daf`.
2. Cargo will download, build, and install the _daf_ binary.
3. The binary will be placed in `$HOME/.cargo` on macOS or Linux, or `%USERPROFILE%\.cargo\bin` on Windows.
4. Verify that _daf_ is installed correctly by running the command `daf --version`.

## Usage

To use `daf`, run the following command:

```
daf.exe [OPTIONS] [PATH]
```

where `[PATH]` is the directory/file path to use.

## Options

`daf` provides several options to customize the output format:

- `-c, --columns`: Sets the output format to columns
- `-h, --help`: Print help information
- `-j, --json`: Sets the output format to JSON
- `-l, --list`: Sets the output format to list
- `-t, --table`: Sets the output format to table
- `-V, --version`: Print version information
- `-y, --yaml`: Sets the output format to YAML

## Examples

Here are some examples of how to use `daf`:

- To list the directories and files in the current directory in a column format:

```
daf.exe -c .
```

- To list the directories and files in a specific directory in a JSON format:

```
daf.exe -j path/to/directory
```

- To print the version information of `daf`:

```
daf.exe -V
```

- To list the directories and files in the current directory in a list format:

```
daf.exe -l .
```

- To list the directories and files in a specific directory in a table format:

```
daf.exe -t path/to/directory
```

- To list the directories and files in a specific directory in a YAML format:

```
daf.exe -y path/to/directory
```

- To print the help information of `daf`:

```
daf.exe -h
```

I hope these examples help you understand how to use the different options of `daf`. 

## Build

To build the `Directories & Files (daf)` CLI app, follow these steps:

1. Clone the `daf` repository from GitHub by running the following command:

```
git clone https://github.com/isurfer21/daf.git
```

2. Navigate to the cloned repository by running the following command:

```
cd daf
```

3. Build the project using Cargo by running the following command:

```
cargo build
```

After running these commands, you should have a local copy of the `daf` project that is ready to use.

The executable binary can be found in the `.\target\debug\` directory. On Windows, the binary is named `daf.exe`, while on macOS it is simply named `daf`.

## Publish

To publish the `daf` crate to [crates.io](https://crates.io/), run the following command:

```
cargo publish
```

This will upload the crate to [crates.io](https://crates.io/) so that others can easily download and use it.