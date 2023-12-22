type Block = {
    row: number;
    col: number;
};

function part1(input: number[][]): number {

    const pos: Block = { row: 0, col: 0 }
    var visited: Set<Block> = new Set([pos]);
    var toVisit: Array<Block> = new Array(pos);

    console.log(visited);
    console.log(toVisit);

    return 5;
}

const input = await Deno.readTextFile("sample.txt");
const grid: number[][] = input.split('\n')
    .map((line: string) =>
        [...line].map((n) => Number(n))
    );
console.log(grid);
part1(grid)
