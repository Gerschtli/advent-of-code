# Advent of Code

Yes, it's christmas time, my friends!

These are my solutions to the [adventofcode.com](https://adventofcode.com/) challenges.

My objective is not only to solve these challenges but to develop them test-driven and in a manner comparable to a
software artifact that has to be understood and maintained for a long time.

"But why?" you ask? Because everybody can write a nice one-liner nobody can understand (including the author after a few
days) and of course for the most important reason: Because I can :D

## How to run

You need go 1.15, nix (at least 2.4 with flakes and nix-command enabled) and rustup (with a recent rust version)
installed or use my `shell.nix`.

### Go applications

Can be run like:

```sh
cd 2020/01
go build -o main
./main
```

### Nix applications

Can be run like:

```sh
cd 2021/01
nix run
```

### Python applications

Can be run like:

```sh
cd 2021/02
pipenv install --deploy
pipenv run app
```

### Rust applications

Can be run like:

```sh
cd 2020/02
cargo run
```
