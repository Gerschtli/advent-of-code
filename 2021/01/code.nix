{ pkgs }:

let
  readFileToInts = file:
    let
      inherit (pkgs.lib) fileContents splitString toInt;

      lines = splitString "\n" (fileContents file);
    in
    map toInt lines;

  countIncreases = numbers:
    let
      inherit (builtins) head tail;
      inherit (pkgs.lib) foldl;

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

  countIncreasesOfFile = file: countIncreases (readFileToInts file);

  countSlidingWindowIncreases = file:
    let
      inherit (builtins) elemAt genList length;

      numbers = readFileToInts file;
      getNumber = elemAt numbers;

      windowSums = genList
        (n: (getNumber n) + (getNumber (n + 1)) + (getNumber (n + 2)))
        (length numbers - 2);
    in
    countIncreases windowSums;
in

{
  tests = [
    {
      description = "count of increments of example input is 7";
      actual = countIncreasesOfFile ./input-example.txt;
      expected = 7;
    }

    {
      description = "count of increments of input.txt is 1292";
      actual = countIncreasesOfFile ./input.txt;
      expected = 1292;
    }

    {
      description = "count of sliding window increments of example input is 5";
      actual = countSlidingWindowIncreases ./input-example.txt;
      expected = 5;
    }

    {
      description = "count of sliding window increments of input.txt is 1262";
      actual = countSlidingWindowIncreases ./input.txt;
      expected = 1262;
    }
  ];

  solution = ''
    Count of larger measurements: ${toString (countIncreasesOfFile ./input.txt)}
    Count of larger measurements within sliding window: ${toString (countSlidingWindowIncreases ./input.txt)}
  '';
}
