{ pkgs, helpers }:

let
  inherit (builtins)
    add
    elemAt
    head
    length
    tail
    ;
  inherit (pkgs.lib)
    fileContents
    init
    groupBy
    last
    foldl
    splitString
    stringToCharacters
    toInt
    zipListsWith
    ;

  readFileToStrings = file: splitString "\n" (fileContents file);
  splitNumberStringToDigits = str: map toInt (stringToCharacters str);
  countOnesInPosition = listOfNumbers:
    foldl
      (zipListsWith add)
      (head listOfNumbers)
      (tail listOfNumbers);

  calculateRates = length: oneCounts:
    let
      mapRange = mostCommon: leastCommon:
        map
          (count:
            if count * 2 > length
            then mostCommon
            else if count * 2 == length
            then throw "no most/least common digit found (length: ${toString length}, oneCounts: ${toString oneCounts})"
            else leastCommon
          )
          oneCounts;
    in
    {
      gamma = mapRange 1 0;
      epsilon = mapRange 0 1;
    };

  binaryToDecimal = binary:
    let
      bitsToInt = binary:
        if binary == [ ]
        then 0
        else (2 * (bitsToInt (init binary))) + last binary;
    in
    bitsToInt binary;

  evaluateReport = file:
    let
      listOfNumbers = map splitNumberStringToDigits (readFileToStrings file);
      oneCounts = countOnesInPosition listOfNumbers;
      binaryRates = calculateRates (length listOfNumbers) oneCounts;
    in
    {
      gamma = binaryToDecimal binaryRates.gamma;
      epsilon = binaryToDecimal binaryRates.epsilon;
    };

  calculateRating = listOfNumbers: index: oxygen:
    let
      grouped = groupBy (binary: toString (elemAt binary index)) listOfNumbers;

      count1 = length grouped."1";
      count0 = length grouped."0";

      newList =
        if (oxygen && count1 >= count0) || (!oxygen && count1 < count0)
        then grouped."1"
        else grouped."0";
    in
    if length listOfNumbers == 1
    then elemAt listOfNumbers 0
    else calculateRating newList (index + 1) oxygen;


  evaluateReport2 = file:
    let
      listOfNumbers = map splitNumberStringToDigits (readFileToStrings file);
      oneCounts = countOnesInPosition listOfNumbers;
      binaryRatings = calculateRates (length listOfNumbers) oneCounts;
    in
    {
      oxygen = binaryToDecimal (calculateRating listOfNumbers 0 true);
      co2 = binaryToDecimal (calculateRating listOfNumbers 0 false);
    };
in

{
  tests = {
    "test that readFileToStrings returns lines as list" = {
      expr = readFileToStrings ./files/input-example.txt;
      expected = [ "00100" "11110" "10110" "10111" "10101" "01111" "00111" "11100" "10000" "11001" "00010" "01010" ];
    };

    "test that splitNumberStringToDigits returns list of digits" = {
      expr = splitNumberStringToDigits "00101";
      expected = [ 0 0 1 0 1 ];
    };

    "test that countOnesInPosition returns first list item if list only has one item" = {
      expr = countOnesInPosition [
        [ 0 1 0 ]
      ];
      expected = [ 0 1 0 ];
    };

    "test that countOnesInPosition returns counts of digit 1 in the respective position" = {
      expr = countOnesInPosition [
        [ 0 1 0 ]
        [ 1 1 0 ]
        [ 1 1 0 ]
      ];
      expected = [ 2 3 0 ];
    };

    "test that calculateRates returns gamma and epsilon rates based on one counts and odd overall length" = {
      expr = calculateRates 3 [ 0 1 2 3 ];
      expected = {
        gamma = [ 0 0 1 1 ];
        epsilon = [ 1 1 0 0 ];
      };
    };

    "test that calculateRates throws error when one position has same count of ones and zeros" =
      helpers.assertExceptionIsThrown
        (calculateRates 2 [ 1 ]);

    "test that binaryToDecimal converts 0 to 0" = {
      expr = binaryToDecimal [ 0 ];
      expected = 0;
    };

    "test that binaryToDecimal converts 101 to 5" = {
      expr = binaryToDecimal [ 1 0 1 ];
      expected = 5;
    };

    "test that evaluateReport for example data returns gamma and epsilon rates" = {
      expr = evaluateReport ./files/input-example.txt;
      expected = {
        gamma = 22;
        epsilon = 9;
      };
    };

    "test that evaluateReport for input data returns gamma and epsilon rates" = {
      expr = evaluateReport ./files/input.txt;
      expected = {
        gamma = 1776;
        epsilon = 2319;
      };
    };

    "test that evaluateReport2 for example data returns oxygen and CO2 ratings" = {
      expr = evaluateReport2 ./files/input-example.txt;
      expected = {
        oxygen = 23;
        co2 = 10;
      };
    };

    "test that evaluateReport2 for input data returns oxygen and CO2 ratings" = {
      expr = evaluateReport2 ./files/input.txt;
      expected = {
        oxygen = 1527;
        co2 = 2510;
      };
    };
  };

  solution = ''
    part 1: ${
      let
        rates = evaluateReport ./files/input.txt;
      in
      toString (rates.gamma * rates.epsilon)
    }
    part 2: ${
      let
        ratings = evaluateReport2 ./files/input.txt;
      in
      toString (ratings.oxygen * ratings.co2)
    }
  '';
}
