import { Block, PriorityQueue } from './priority_queue.ts';

const delta: [number, number][] = [
    [0, 1],
    [1, 0],
    [0, -1],
    [-1, 0]
];

function part1(grid: number[][]): number {

    const pos: Block = {
        heat: 0,
        r: 0,
        c: 0,
        dr: 0,
        dc: 0,
        n: 0
    };

    const inBound = (r: number, c: number) => 0 <= r && r < grid.length && 0 <= c && c < grid[0].length;

    var visited: Set<Block> = new Set();
    const toVisit = new PriorityQueue();
    toVisit.enqueue(pos);

    while (!toVisit.isEmpty()) {
        const b: Block = toVisit.dequeue();
        const { heat, r, c, dr, dc, n } = b;

        if (r === grid.length - 1 && c === grid[0].length - 1) {
            return heat;
        }

        const hasVisited = Array.from(visited).some((block) => visitedBlock(block, b));
        if (hasVisited) continue
        visited.add(b);

        if (n < 3 && (dr !== 0 || dc !== 0)) {
            const nr: number = r + dr;
            const nc: number = c + dc;

            if (inBound(nr, nc)) {
                const next: Block = {
                    heat: heat + grid[nr][nc],
                    r: nr,
                    c: nc,
                    dr: dr,
                    dc: dc,
                    n: n + 1
                }
                toVisit.enqueue(next);
            }
        }

        for (const [ndr, ndc] of delta) {
            if (!(dr === ndr && dc === ndc) && !(-dr === ndr && -dc === ndc)) {
                const nr: number = r + ndr;
                const nc: number = c + ndc;

                if (inBound(nr, nc)) {
                    const next: Block = {
                        heat: heat + grid[nr][nc],
                        r: nr,
                        c: nc,
                        dr: ndr,
                        dc: ndc,
                        n: 1
                    }
                    toVisit.enqueue(next);
                }

            }
        }
    }
    return -1;
}

const visitedBlock = (lhs: Block, rhs: Block): boolean => {
    return lhs.r === rhs.r &&
        lhs.c === rhs.c &&
        lhs.dr === rhs.dr &&
        lhs.dc === rhs.dc &&
        lhs.n === rhs.n;
};

const input = await Deno.readTextFile("input");
const grid: number[][] = input
    .split('\n')
    .filter((line: string) => line.trim() !== '')
    .map((line: string) =>
        [...line].map((n) => Number(n))
    );

console.log("Part 1: ", part1(grid));
