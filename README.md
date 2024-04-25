# Building
`cargo build --release`\
The built binaries will be located in `./target/release/`

# Installing
```console
cd
git clone https://github.com/kivattt/tutils
cd tutils
cargo build --release
```
Then add this to your `.bashrc` file, and re-open a terminal
```bash
tutilspath=~/tutils/target/release
if test -d $tutilspath; then
        alias ls="$tutilspath/ls"
        alias pwd="$tutilspath/pwd"
        alias cat="$tutilspath/cat"
        alias hex="$tutilspath/hex"
        alias xxd="$tutilspath/xxd"
else
        echo "Could not find tutils programs in $tutilspath"
fi
```

Since adding `tutils` to your path environment variable could break existing scripts that rely on system utilities specific behaviour, we use shell aliases so that shell scripts will continue to use the existing utilities, rather than `tutils`.


# Commands
`cat` Print a file or STDIN\
`hex` Encode/decode hex\
`ls` List files\
`pwd` Print working directory\
`xxd` Visualize as hex
