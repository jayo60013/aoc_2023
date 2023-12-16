<?php

// printf('Part 1: %d'.PHP_EOL, part1('input'));
printf('Part 2: %d'.PHP_EOL, part2('input'));

const EXPANSION_RATE = 10;

function part1($filename)
{
    $grid = expandGrid(file($filename));
    $grid = transposeGrid($grid);
    $grid = expandGrid($grid);

    $galaxies = getGalaxies($grid);

    return getSumOfDistances($galaxies);
}

function getSumOfDistances($galaxies)
{
    $sum = 0;
    for ($i = 0; $i < count($galaxies); ++$i) {
        for ($j = $i + 1; $j < count($galaxies); ++$j) {
            if ($galaxies[$i] == $galaxies[$j]) {
                continue;
            }
            $sum += getDistance($galaxies[$i], $galaxies[$j]);
        }
    }

    return $sum;
}

function getDistance($lhs, $rhs)
{
    return abs($lhs[0] - $rhs[0]) + abs($lhs[1] - $rhs[1]);
}

function getGalaxies($grid)
{
    $galaxies = [];
    $row = 0;
    foreach ($grid as $line) {
        $lastPos = 0;
        while (($lastPos = strpos($line, '#', $lastPos)) !== false) {
            $galaxies[] = [$row, $lastPos];
            ++$lastPos;
        }
        ++$row;
    }

    return $galaxies;
}

function expandGrid($grid)
{
    $new_grid = [];
    foreach ($grid as $line) {
        $new_grid[] = $line;
        if (!str_contains($line, '#')) {
            $new_grid[] = str_repeat('.', strlen($line) - 1);
        }
    }

    return $new_grid;
}

function transposeGrid($arr)
{
    $arr = array_map(function ($string) {
        return str_split($string);
    }, $arr);

    $transposed = array_map(null, ...$arr);
    $repackaged = array_map(function ($chars) {
        return implode('', $chars);
    }, $transposed);
    array_pop($repackaged);

    return $repackaged;
}

function part2($filename)
{
    $grid = file($filename);
    $empty_rows = getEmptyRows($grid);
    $empty_cols = getEmptyRows(transposeGrid($grid));

    $galaxies = getGalaxies($grid);

    return getSummedDistance($galaxies, $empty_rows, $empty_cols);
}

function getEmptyRows($grid)
{
    $rows = [];
    for ($i = 0; $i < count($grid); ++$i) {
        if (!str_contains($grid[$i], '#')) {
            $rows[] = $i;
        }
    }

    return $rows;
}

function getSummedDistance($galaxies, $empty_rows, $empty_cols)
{
    $EXPANSION_RATE = 2;
    $sum = 0;
    for ($i = 0; $i < count($galaxies); ++$i) {
        for ($j = $i + 1; $j < count($galaxies); ++$j) {
            if ($galaxies[$i] == $galaxies[$j]) {
                continue;
            }

            $lhs = [
              min($galaxies[$i][0], $galaxies[$j][0]),
              min($galaxies[$i][1], $galaxies[$j][1]),
            ];
            $rhs = [
              max($galaxies[$i][0], $galaxies[$j][0]),
              max($galaxies[$i][1], $galaxies[$j][1]),
            ];

            $additional_rows = 0;
            foreach ($empty_rows as $row) {
                if ($lhs[0] < $row and $row < $rhs[0]) {
                    ++$additional_rows;
                }
            }

            $additional_cols = 0;
            foreach ($empty_cols as $col) {
                if ($lhs[1] < $col and $col < $rhs[1]) {
                    ++$additional_cols;
                }
            }

            $sum += ($rhs[0] + ($additional_rows * ($EXPANSION_RATE - 1)) - $lhs[0])
              + ($rhs[1] + ($additional_cols * ($EXPANSION_RATE - 1)) - $lhs[1]);
        }
    }

    return $sum;
}
