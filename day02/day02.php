<?php

printf('Part 1: %d'.PHP_EOL, part1('input'));
printf('Part 2: %d'.PHP_EOL, part2('input'));

function part2($file)
{
    $sum = 0;

    foreach (file($file) as $line) {
        $red = 0;
        $blue = 0;
        $green = 0;
        $games = explode(';', $line);

        foreach ($games as $game) {
            preg_match_all(
                '/(\d+) (red|green|blue)/',
                $game,
                $matches,
                PREG_OFFSET_CAPTURE
            );

            foreach ($matches[0] as $match) {
                $quantity = (int) explode(' ', $match[0])[0];
                $color = explode(' ', $match[0])[1];
                switch ($color) {
                    case 'red':
                        $red = max($red, $quantity);
                        break;
                    case 'green':
                        $green = max($green, $quantity);
                        break;
                    case 'blue':
                        $blue = max($blue, $quantity);
                        break;
                    default:
                        exit("Unknown color ($color)");
                }
            }
        }
        $sum += $red * $green * $blue;
    }

    return $sum;
}

function part1($file)
{
    $id = 1;
    $sum = 0;

    foreach (file($file) as $line) {
        $red = false;
        $blue = false;
        $green = false;

        $games = explode(';', $line);

        foreach ($games as $game) {
            preg_match_all(
                '/(\d+) (red|green|blue)/',
                $game,
                $matches,
                PREG_OFFSET_CAPTURE
            );

            foreach ($matches[0] as $match) {
                $quantity = explode(' ', $match[0])[0];
                $color = explode(' ', $match[0])[1];
                switch ($color) {
                    case 'red':
                        $red |= ($quantity > 12);
                        break;
                    case 'green':
                        $green |= ($quantity > 13);
                        break;
                    case 'blue':
                        $blue |= ($quantity > 14);
                        break;
                    default:
                        exit("Unknown color ($color)");
                }
            }
        }
        if (!($red or $green or $blue)) {
            $sum += $id;
        }
        ++$id;
    }

    return $sum;
}
