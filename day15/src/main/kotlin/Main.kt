import java.nio.file.Files
import java.nio.file.Paths

fun main() {
    val input = parseInput("input")
    println("Part 1: ${input.sumOf { hashStep(it) }}")
    println("Part 2: ${part2(input)}")
}

data class Lens(val label: String, var focal: Int)

fun part2(input: List<String>): Int {
    val boxes: Array<MutableList<Lens>> = Array(256) { mutableListOf() }
    val labelRegex = Regex("^[a-z]+")

    input.forEach { step ->
        val label = labelRegex.find(step)?.value ?: ""
        val i = hashStep(label)

        when {
            step.contains("=") -> {
                val focal = step.split("=")[1].toInt()
                val lens = boxes[i].find { it.label == label }

                if (lens != null) {
                    lens.focal = focal
                } else {
                    boxes[i].add(Lens(label, focal))
                }
            }

            step.contains("-") -> {
                boxes[i].removeIf { it.label == label }
            }
        }
    }

    return boxes.indices.sumOf { i ->
        boxes[i].indices.sumOf { j ->
            (i + 1) * (j + 1) * boxes[i][j].focal
        }
    }
}

fun hashStep(step: String): Int =
    step.fold(0) { acc, ch -> (acc + ch.code) * 17 % 256 }

fun parseInput(filename: String): List<String> {
    val lines = Files.readAllLines(Paths.get(filename))
    return lines[0].trim().split(",")
}
