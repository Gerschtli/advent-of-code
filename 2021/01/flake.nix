{
  description = "advent-of-code-2021-01";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      success = pkgs.runCommand "test-result" { } "touch $out";
      fail = pkgs.runCommand "test-result" { } "exit 1";

      evalTests = tests:
        let
          inherit (builtins) length trace;
          inherit (pkgs.lib) concatMapStringsSep foldl optional;

          failedTests = foldl
            (failedTests: { actual, expected, ... } @ test:
              failedTests ++ optional (actual != expected) test
            )
            [ ]
            tests;
        in
        if failedTests == [ ]
        then success
        else
          trace
            ''
              ${toString (length failedTests)} tests failed!

              ${concatMapStringsSep "\n" ({ description, actual, expected }: ''
                > ${description}
                    actual:   ${toString actual}
                    expected: ${toString expected}
              '') failedTests}
            ''
            fail;

      code = import ./code.nix { inherit pkgs; };
    in
    {
      checks.${system}.tests = evalTests code.tests;

      defaultApp.${system} = {
        type = "app";
        program = "${pkgs.writeScript "solution" ''
          cat ${pkgs.writeText "solution-content" code.solution}
        ''}";
      };
    };
}
