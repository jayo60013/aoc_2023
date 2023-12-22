type Block = {
    heat: number;
    r: number;
    c: number;
    dr: number;
    dc: number;
    n: number;
};

function part1(input: number[][]): number {

    const pos: Block = {
        heat: 0,
        r: 0,
        c: 0,
        dr: 0,
        dc: 0,
        n: 0
    };
    var visited: Set<Block> = new Set([pos]);
    var toVisit: Array<Block> = new Array(pos);

    while (toVisit.length > 0) {
        const b: Block = toVisit.pop();

        if (b.r < 0 || b.r >= input.length || b.c < 0 || b.c >= input[0].length)
            continue

        const hasVisited = Array.from(visited).some((block) => visitedBlock(block, b));
        if (hasVisited) continue
    }
    return 5;
}

const visitedBlock = (lhs: Block, rhs: Block): boolean => {
    return lhs.r === rhs.r &&
        lhs.c === rhs.c &&
        lhs.dr === rhs.dr &&
        lhs.dc === rhs.dc &&
        lhs.n === rhs.n;
};

const input = await Deno.readTextFile("sample.txt");
const grid: number[][] = input.split('\n')
    .map((line: string) =>
        [...line].map((n) => Number(n))
    );
part1(grid)
