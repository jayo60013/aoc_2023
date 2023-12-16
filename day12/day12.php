<?php

printf("Part 1: %d\n", part1(file('sample.txt')));
// printf('Part 2: %d'.PHP_EOL, part2(file('sample.txt')));

function part1($input)
{
    $sum = 0;
    foreach ($input as $line) {
        [$pattern, $arrangement] = explode(' ', trim($line));
        $arrangement = array_map('intVal', explode(',', $arrangement));

        $num = substr_count($pattern, '?');

        $p = 0;
        while ($p < pow(2, $num)) {
            $perm = sprintf('%0'.$num.'s', decbin($p));
            $newLine = '';
            $permIdx = 0;

            for ($i = 0; $i < strlen($pattern); ++$i) {
                if ('?' != $pattern[$i]) {
                    $newLine .= $pattern[$i];
                } else {
                    $newLine .= ('0' == $perm[$permIdx] ? '.' : '#');
                    ++$permIdx;
                }
            }
            if (checkArrangement($newLine, $arrangement)) {
                ++$sum;
            }
            ++$p;
        }
    }

    return $sum;
}

function checkArrangement($line, $arrangement)
{
    $re = '/(#+)/';
    preg_match_all($re, $line, $matches);
    $t = array_map(function ($n) {
        return strlen($n);
    }, $matches[0]);

    return $t == $arrangement;
}
