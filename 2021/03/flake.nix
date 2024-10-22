{
  description = "advent-of-code-2021-03";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    import ../../lib/nix/flake-builder.nix {
      inherit nixpkgs;
      appFile = ./app.nix;
    };
}
