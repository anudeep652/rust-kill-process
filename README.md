<div align="center">

# Rust kill Process

</div>

A simple CLI tool to kill a process by port
This is a simple hobby project made by me to gain some experience in working with rust.
<br>
I felt exhaused by using `kill -9 $(lsof -t -i:PORT)` everytime I wanted to kill a process by port, so I made this tool.
<br>
<br>

## Usage

clone the repo:

```sh
git clone https://github.com/anudeep652/rust-kill-process.git
```

<br>

```sh
cd rust-kill-process
```

```sh
cargo run -- PORT
```

> Remember to replace PORT with the port number you want to kill

<br>
OR
<br>
<br>
Create an executable to use it locally anywhere after cloning the repo

```sh
cargo install --path .
```

Now you can run anywhere in your terminal like this:

```sh
p-kill PORT
```

> 'p-kill' is the executable name, you can change it in Cargo.toml and you need to again create an executable.

<br>

## Future plans

- Add support for killing process by name
- Add support for killing process by PID
- Add better error handling

<br>

## Contributing

If you are a rustacean and want to contribute to this project, feel free to open a PR.
