import java.nio.file.Files
import java.nio.file.Path

fun main() {
    val filename = "input"
    val input = Files.readAllLines(Path.of(filename))
    println("Part 1: ${part1(input)}")
    println("Part 2: ${part2(input)}")
}

data class Beam(val r: Int, val c: Int, val dr: Int, val dc: Int)

fun part1(input: List<String>): Int {
    val startingPos = Beam(0, -1, 0, 1)
    return solve(input, startingPos)
}

fun part2(input: List<String>): Int {
    val edges = listOf(
        input.indices.map { Beam(it, -1, 0, 1) },
        input.indices.map { Beam(it, input[0].length, 0, -1) },
        (0..<input[0].length).map { Beam(-1, it, 1, 0) },
        (0..<input[0].length).map { Beam(input.size, it, -1, 0) }
    ).flatten()

    return edges.maxOf { solve(input, it) }
}

fun solve(input: List<String>, startingPos: Beam): Int {
    val visited = mutableSetOf<Beam>()
    val toVisit = mutableListOf<Beam>()
    toVisit.add(startingPos)

    while (toVisit.isNotEmpty()) {
        var (r, c, dr, dc) = toVisit.removeFirst()
        r += dr
        c += dc

        if (r !in input.indices || c !in input[0].indices) continue
        val sym = input[r][c]

        if (sym == '.' ||
            (sym == '-' && dc != 0) ||
            (sym == '|' && dr != 0)
        ) {
            val b = Beam(r, c, dr, dc)
            if (visited.add(b)) toVisit.add(b)
            continue
        }

        when (sym) {
            '/' -> {
                dr = -dc.also { dc = -dr }
                val b = Beam(r, c, dr, dc)
                if (visited.add(b)) toVisit.add(b)
            }

            '\\' -> {
                dr = dc.also { dc = dr }
                val b = Beam(r, c, dr, dc)
                if (visited.add(b)) toVisit.add(b)
            }

            '|' -> {
                val up = Beam(r, c, -1, 0)
                if (visited.add(up)) toVisit.add(up)

                val down = Beam(r, c, 1, 0)
                if (visited.add(down)) toVisit.add(down)
            }

            '-' -> {
                val left = Beam(r, c, 0, -1)
                if (visited.add(left)) toVisit.add(left)

                val right = Beam(r, c, 0, 1)
                if (visited.add(right)) toVisit.add(right)
            }
        }
    }
    return visited.map { it.r to it.c }.toSet().size
}
