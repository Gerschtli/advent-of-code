{ nixpkgs, appFile }:

let
  system = "x86_64-linux";
  pkgs = nixpkgs.legacyPackages.${system};

  success = pkgs.runCommand "test-result" { } "touch $out";
  fail = pkgs.runCommand "test-result" { } "exit 1";

  evalTests = tests:
    let
      inherit (builtins)
        genList
        length
        replaceStrings
        trace
        typeOf
        ;
      inherit (pkgs.lib)
        concatMapStringsSep
        foldl
        generators
        id
        runTests
        ;

      failedTests = runTests tests;

      printValue = indention: value:
        replaceStrings
          [ "\n" ]
          [ (foldl (a: _: a + " ") "\n" (genList id indention)) ]
          (generators.toPretty { allowPrettyValues = true; } value);

      printStatement = description: value: ''
        --> ${description}: (type: ${typeOf value})
            ${printValue 4 value}
      '';
    in
    if failedTests == [ ]
    then success
    else
      trace
        ''
          ${toString (length failedTests)} tests failed!

          ${concatMapStringsSep "\n" ({ name, result, expected }: ''
            > ${name}

            ${printStatement "result" result}
            ${printStatement "expected" expected}
          '') failedTests}
        ''
        fail;

  helpers = {
    assertExceptionIsThrown = expr: {
      expr = builtins.tryEval (builtins.deepSeq expr expr);
      expected = {
        success = false;
        value = false;
      };
    };
  };

  app = import appFile { inherit helpers pkgs; };
in

{
  checks.${system}.tests = evalTests app.tests;

  apps.${system}.default = {
    type = "app";
    program = "${pkgs.writeScript "solution" ''
      cat ${pkgs.writeText "solution-content" app.solution}
    ''}";
  };
}
