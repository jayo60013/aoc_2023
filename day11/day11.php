<?php

$grid = file('input');
$empty_rows = getEmptyRows($grid);
$empty_cols = getEmptyRows(transposeGrid($grid));

$galaxies = getGalaxies($grid);

printf('Part 1: %d'.PHP_EOL, getSummedDistance($galaxies, $empty_rows, $empty_cols, 2));
printf('Part 2: %d'.PHP_EOL, getSummedDistance($galaxies, $empty_rows, $empty_cols, 1_000_000));

function getGalaxies($grid)
{
    $galaxies = [];

    foreach ($grid as $row => $line) {
        $positions = array_keys(str_split($line, 1), '#');
        foreach ($positions as $col) {
            $galaxies[] = [$row, $col];
        }
    }

    return $galaxies;
}

function transposeGrid($arr)
{
    $arr = array_map(function ($string) {
        return str_split($string);
    }, $arr);

    $transposed = array_map(null, ...$arr);

    return array_map(function ($chars) {
        return implode('', $chars);
    }, $transposed);
}

function getEmptyRows($grid)
{
    return array_keys(array_filter($grid, function ($line) {
        return !str_contains($line, '#');
    }));
}

function getSummedDistance($galaxies, $empty_rows, $empty_cols, $expansion)
{
    $sum = 0;
    for ($i = 0; $i < count($galaxies); ++$i) {
        for ($j = $i + 1; $j < count($galaxies); ++$j) {
            if ($galaxies[$i] == $galaxies[$j]) {
                continue;
            }
            $minRow = min($galaxies[$i][0], $galaxies[$j][0]);
            $minCol = min($galaxies[$i][1], $galaxies[$j][1]);

            $maxRow = max($galaxies[$i][0], $galaxies[$j][0]);
            $maxCol = max($galaxies[$i][1], $galaxies[$j][1]);

            $extra_rows = array_reduce(
                $empty_rows,
                function ($carry, $row) use ($minRow, $maxRow) {
                    return $carry + (($minRow < $row and $row < $maxRow) ? 1 : 0);
                },
                0
            );

            $extra_cols = array_reduce(
                $empty_cols,
                function ($acc, $col) use ($minCol, $maxCol) {
                    return $acc + (($minCol < $col and $col < $maxCol) ? 1 : 0);
                },
                0
            );

            $sum += ($maxRow + ($extra_rows * ($expansion - 1)) - $minRow)
              + ($maxCol + ($extra_cols * ($expansion - 1)) - $minCol);
        }
    }

    return $sum;
}
