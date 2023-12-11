import java.nio.file.Files
import java.nio.file.Paths

fun main(args: Array<String>) {
    val (instructions, map, starting) = parseFile("input")
    println("Part 1: ${part1(instructions, map)}")
    println("Part 2: ${part2(instructions, map, starting.toMutableList())}")
}

fun parseFile(filename: String): Triple<String, Map<String, Pair<String, String>>, List<String>> {
    val lines = Files.readAllLines(Paths.get(filename))
    val instructions = lines[0]
    val input = mutableMapOf<String, Pair<String, String>>()
    val starting = mutableListOf<String>()

    lines.drop(2).forEach { line ->
        val (key, left, right) = """^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)""".toRegex()
            .matchEntire(line)!!.groupValues.drop(1)
        input[key] = Pair(left, right)
        if (key.endsWith("A")) starting.add(key)
    }

    return Triple(instructions, input, starting)
}

fun part1(instructions: String, map: Map<String, Pair<String, String>>): Int {
    var cur = "AAA"
    var count = 0

    while (cur != "ZZZ") {
        val (left, right) = map[cur] ?: Pair("", "")
        cur = if (instructions[count % instructions.length] == 'L') {
            left
        } else {
            right
        }
        count += 1
    }
    return count
}

fun part2(instructions: String, map: Map<String, Pair<String, String>>, nodes: MutableList<String>): Long {
    val counts = mutableListOf<Long>()

    for (i in nodes.indices) {
        var count = 0
        while (!nodes[i].endsWith("Z")) {
            nodes[i] = if (instructions[count % instructions.length] == 'L') {
                map[nodes[i]]?.first.toString()
            } else {
                map[nodes[i]]?.second.toString()
            }
            count += 1
        }
        counts.add(count.toLong())
    }
    return counts.reduce { acc, num -> lcm(acc, num) }
}

fun gcd(a: Long, b: Long): Long =
    if (b == 0L) a else gcd(b, a % b)

fun lcm(a: Long, b: Long): Long =
    (a * b) / gcd(a, b)