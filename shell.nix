{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = [
    pkgs.cargo-edit
    pkgs.go
    pkgs.golangci-lint
    pkgs.pipenv
    pkgs.python310
    pkgs.rustup
  ];
}
