// Convert the contents of the file to an array of arrays
// where each line is an array of ints
const getValues = (lines) => {
    return lines
        .filter(line => line.trim() !== '')
        .map((line) => {
            const pairs = line
                .split(" ")
                .map((n) => parseInt(n));
            return pairs
        });
}

// Convert a line from the input to an array of differences until every element is 0
const getHistory = (line) => {
    const history = [line];

    while (!line.every((n) => n === 0)) {
        line = line.slice(1).map((x, i) => x - line[i]);
        history.push(line);
    }
    return history.reverse();
};


async function part1(lines) {
    // Use the history to get the next value, by summing the last element
    const getNext = (history) => {
        return history.slice(0, -1)
            .map((_, i) => history[i + 1].slice(-1)[0])
            .reduce((acc, curr) => acc + curr, 0);
    }

    // create an array of histories from the input and sum up the next values
    return getValues(lines).map(getHistory)
        .reduce((acc, history) => acc + getNext(history), 0);
}

async function part2(lines) {
    // Use the history to get the next value, by summing the last element
    const getPrev = (history) => {
        return history.slice(0, -1)
            .map((_, i) => history[i + 1][0])
            .reduce((acc, curr) => curr - acc, 0);
    }

    return getValues(lines).map(getHistory)
        .reduce((acc, history) => acc + getPrev(history), 0);
}

const content = await Deno.readTextFile("input");
const lines = content.split("\n");

console.log("Part 1:", await part1(lines));
console.log("Part 2:", await part2(lines));
