# File Search

Program to run similar to Unix based `find` command. Written in rust with limited feature set.

## Build

First make sure you have rust installed and are running the latest version. If rust is installed run:
```bash
rustup update
```

To create the binary run the following: `cargo build --release`

This will create the binary in the following dir: `./target/release/search`

## Features

You can check the supported arguments running the bin `./target/release/search --help`

It asks that you pass in a starting directory as the first argument otherwise it defaults to the current directory

### --name

This argument is **required** but if you want to do a full system search you can simply pass *.

We also support wildcard matching for files such as `search . --name "main.*"` to search any file or dir named "main." with any extension

### --type

This expects one of two args: `--type f` or `type d` which will specify file or dir search. If you wish for both simply don't supply this argument.

### --max-open

This allows for multiple paths to be searched simultaniously at once. **The default value used is `3`**

Use this sparringly as the higher the number the more CPU power is used.

### --exclude

We pass in a comma separated string for this one to allow multiple directories to be used:
- Example: `search --name "main.rs" --exclude "/home/user/Downloads,/home/user/Pictures"` 
- ***Notice:*** We use the absolute path. While simple directory names would be nice this would lead to any directory with the same name to also be excluded.

***PLEASE NOTE:*** **Additional testing is needed. Currently the speed is difficult to match as find command is not specific on path exclusion**
