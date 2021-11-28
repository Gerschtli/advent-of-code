{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = [
    pkgs.cargo-edit
    pkgs.go_1_15
    pkgs.golangci-lint
    pkgs.rustup
  ];
}
