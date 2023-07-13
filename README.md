# mgrep
A command line tool that like grep. you cloud use it to search message.

# usage

### build executable file in project root path.
```
cargo build --release
```

### use mgrep to query the specified message.
the syntax:
```
mgrep <query> <file path>
```
example:
```
./target/release/mgrep to poem.txt
Are you nobody, too?
How dreary to be somebody!
```
