# Simple Command Line Simulator
================================

A simple command line simulator written in Rust, mimicking some basic Linux commands.

## Features
------------

* Simulates basic Linux commands
* Supports the following commands:
  * `cl echo`: Prints arguments
  * `cl read`: Reads files
  * `cl list`: Lists directories
  * `cl find`: Finds files or directories
  * `cl grep`: Searches for content in input
  * `cl clear`: Clears the command line
  * `cl mkdir`: Creates a new directory
  * `cl pwd`: Prints the working directory
  * `cl rm`: Removes a file or directory
  * `cl whoami`: Prints the current user

## Usage
---------

To use the command line simulator, simply run the executable and type `cl help` to see the available commands.

### Commands

* `cl echo <args>`: Print arguments
* `cl read <args>`: Read files
* `cl list <dir>`: List directories
* `cl find <dir> <type> <name>`: Find files or directories
* `cl grep <content> <input>`: Search for content in input
* `cl clear`: Clear the command line
* `cl mkdir`: Create a new directory
* `cl pwd`: Print the working directory
* `cl rm`: Remove a file or directory
* `cl whoami`: Print the current user

## Requirements
------------

* Rust installed on your system

## Building and Running
----------------------

To build and run the command line simulator, follow these steps:

1. Clone the repository
2. Navigate to the repository directory
3. Run `cargo build` to build the executable
4. Run `cargo run -- <command> <args>` to run the executable with a specific command and arguments

## Contributing
------------

Contributions are welcome! If you'd like to add new features or fix bugs, please submit a pull request.

## License
-------

This project is licensed under the GNU General Public License version 3 (GNU GPL 3).
