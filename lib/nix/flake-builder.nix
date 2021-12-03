{ nixpkgs, appFile }:

let
  system = "x86_64-linux";
  pkgs = nixpkgs.legacyPackages.${system};

  success = pkgs.runCommand "test-result" { } "touch $out";
  fail = pkgs.runCommand "test-result" { } "exit 1";

  evalTests = tests:
    let
      inherit (builtins) length trace;
      inherit (pkgs.lib) concatMapStringsSep foldl optional runTests;

      failedTests = runTests tests;
    in
    if failedTests == [ ]
    then success
    else
      trace
        ''
          ${toString (length failedTests)} tests failed!

          ${concatMapStringsSep "\n" ({ name, result, expected }: ''
            > ${name}
                result:   ${toString result}
                expected: ${toString expected}
          '') failedTests}
        ''
        fail;

  app = import appFile { inherit pkgs; };
in

{
  checks.${system}.tests = evalTests app.tests;

  defaultApp.${system} = {
    type = "app";
    program = "${pkgs.writeScript "solution" ''
      cat ${pkgs.writeText "solution-content" app.solution}
    ''}";
  };
}
