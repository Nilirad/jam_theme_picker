# Jam Theme Picker

A simple utility program to randomly pick up a string among a set provided through command line arguments. It can be particularly useful for picking up a random theme for game jams but it is also suitable for other uses. Source of randomness is provided by [random.org](random.org).

Bevy project template generously provided by [Leafwing Studios](https://github.com/Leafwing-Studios/template-repo).

## Installation

Install this utility on your machine using cargo:

```shell
cargo install jam_theme_picker
```

## Usage

If you want to run the program from the repository, go to the directory of this README file, and run a command with the following form:

```shell
cargo run -- "theme 1" "theme 2" "theme 3"
```

Otherwise, if you installed the utility using `cargo install`, you can simply run from anywhere:

```shell
jam_theme_picker "theme 1" "theme 2" "theme 3"
```

The number of strings must be at least two.