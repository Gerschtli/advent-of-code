{ pkgs }:

let
  countIncreases = file:
    let
      inherit (builtins) head tail;
      inherit (pkgs.lib) fileContents foldl splitString toInt;

      lines = splitString "\n" (fileContents file);
      numbers = map toInt lines;

      increaseCounter = foldl
        ({ last, counter }: num: {
          last = num;
          counter = if num > last then counter + 1 else counter;
        })
        {
          last = head numbers;
          counter = 0;
        }
        (tail numbers);
    in
    increaseCounter.counter;
in

{
  tests = [
    {
      description = "count of increments of example input is 7";
      actual = countIncreases ./input-example.txt;
      expected = 7;
    }

    {
      description = "count of increments of input.txt is 1292";
      actual = countIncreases ./input.txt;
      expected = 1292;
    }
  ];

  solution = ''
    Count of larger measurements: ${toString (countIncreases ./input.txt)}
  '';
}
