<?php

printf("Part 1: %d\n", part1(file('input')));
printf("Part 2: %d\n", part2(file('input')));

function part1($input)
{
    $sum = 0;
    foreach ($input as $line) {
        [$springs, $arrangement] = explode(' ', trim($line));
        $arrangement = array_map('intVal', explode(',', $arrangement));

        $sum += f($springs, $arrangement);
    }

    return $sum;
}

function part2($input)
{
    $sum = 0;
    foreach ($input as $line) {
        [$springs, $arrangement] = explode(' ', trim($line));

        $springs = substr(str_repeat($springs.'?', 5), 0, -1);
        $arrangement = substr(str_repeat($arrangement.',', 5), 0, -1);

        $arrangement = array_map('intVal', explode(',', $arrangement));

        $sum += f($springs, $arrangement);
    }

    return $sum;
}

function f($springs, $numbers, &$memo = [])
{
    // Check if the result is already cached
    $memoKey = $springs.'|'.implode(',', $numbers);
    if (isset($memo[$memoKey])) {
        return $memo[$memoKey];
    }

    // If there are no springs left, and no numbers left: ✅
    // If there are no springs left but there are numbers left: ❌
    if ('' == $springs) {
        return (0 == count($numbers)) ? 1 : 0;
    }

    // If there are no numbers left, and no #'s left: ✅
    // If there are no numbers left but there are #'s left: ❌
    if (0 == count($numbers)) {
        return (str_contains($springs, '#')) ? 0 : 1;
    }

    $result = 0;
    $spring = $springs[0];
    $num = $numbers[0];

    // We can treat the ? like a . in this case and discard the first spring
    if ('.' == $spring or '?' == $spring) {
        $result += f(substr($springs, 1), $numbers, $memo);
    }

    // Means the start of a block
    if ('#' == $spring or '?' == $spring) {
        if (
            // The number of springs we expect has to be less than or equal to the remaining springs
            $num <= strlen($springs)
            // Must be a contigous block of # (or ?)
            and !str_contains(substr($springs, 0, $num), '.')
            // Blocks must be separated by a ., therefore we must be at the end of the springs or springs[num] is a . (or ?)
            and ($num == strlen($springs) or '#' != $springs[$num])
        ) {
            // remove the block at nums + 1 (blocks must be separated by a .)
            $result += f(substr($springs, $num + 1), array_slice($numbers, 1), $memo);
        }
    }

    // Cache the result
    $memo[$memoKey] = $result;

    return $result;
}
