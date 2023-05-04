# df
Directories &amp; Files

**Directories & Files** a.k.a _*df*_, is a command-line interface (CLI) app developed by [Abhishek Kumar](mailto:akbittu@gmail.com). 

This app allows users to list the directories and files in a specified path with or without attributes as opted.

With `df`, users can easily view and manage the contents of their directories and files. The app provides several options to customize the output format, including columns, JSON, list, table, and YAML. This makes it easy for users to view the information in a format that best suits their needs.

`df` is a powerful tool for managing directories and files on your computer. Whether you need to quickly view the contents of a directory or manage your files more efficiently, `df` has got you covered.

## Installation

This guide provides several methods for installing the _df_ tool on your computer. Follow the steps below to set up _df_ using the method that works best for you.

### Method 1: Manual installation

1. Visit the _df_ GitHub page and navigate to the "Releases" section.
2. Download the latest release for your operating system and extract the contents of the archive.
3. Copy the extracted binary file to a directory in your `$PATH`. On macOS or Linux, this could be `/usr/local/bin`, while on Windows, you could copy the binary to `C:\Windows\system32`.
4. Verify that _df_ is installed correctly by opening a terminal or command prompt and running the command `df --version`.

### Method 2: Using Cargo

If you have a Rust development environment set up on your computer, you can use the `cargo install` command to install _df_:

1. Open a terminal or command prompt and run the command `cargo install df`.
2. Cargo will download, build, and install the _df_ binary.
3. The binary will be placed in `$HOME/.cargo` on macOS or Linux, or `%USERPROFILE%\.cargo\bin` on Windows.
4. Verify that _df_ is installed correctly by running the command `df --version`.

## Usage

To use `df`, run the following command:

```
df.exe [OPTIONS] [PATH]
```

where `[PATH]` is the directory/file path to use.

## Options

`df` provides several options to customize the output format:

- `-c, --columns`: Sets the output format to columns
- `-h, --help`: Print help information
- `-j, --json`: Sets the output format to JSON
- `-l, --list`: Sets the output format to list
- `-t, --table`: Sets the output format to table
- `-V, --version`: Print version information
- `-y, --yaml`: Sets the output format to YAML

## Examples

Here are some examples of how to use `df`:

- To list the directories and files in the current directory in a column format:

```
df.exe -c .
```

- To list the directories and files in a specific directory in a JSON format:

```
df.exe -j path/to/directory
```

- To print the version information of `df`:

```
df.exe -V
```

- To list the directories and files in the current directory in a list format:

```
df.exe -l .
```

- To list the directories and files in a specific directory in a table format:

```
df.exe -t path/to/directory
```

- To list the directories and files in a specific directory in a YAML format:

```
df.exe -y path/to/directory
```

- To print the help information of `df`:

```
df.exe -h
```

I hope these examples help you understand how to use the different options of `df`. 

## Build

To build the `Directories & Files (df)` CLI app, follow these steps:

1. Clone the `df` repository from GitHub by running the following command:

```
git clone https://github.com/isurfer21/df.git
```

2. Navigate to the cloned repository by running the following command:

```
cd df
```

3. Build the project using Cargo by running the following command:

```
cargo build
```

After running these commands, you should have a local copy of the `df` project that is ready to use.

The executable binary can be found in the `.\target\debug\` directory. On Windows, the binary is named `df.exe`, while on macOS it is simply named `df`.

## Publish

To publish the `df` crate to [crates.io](https://crates.io/), run the following command:

```
cargo publish
```

This will upload the crate to [crates.io](https://crates.io/) so that others can easily download and use it.