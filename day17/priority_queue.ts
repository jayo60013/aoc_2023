export class PriorityQueue {
    private blocks: Block[] = [];

    enqueue(block: Block) {
        this.blocks.push(block);
        this.blocks.sort((a, b) => a.heat - b.heat);
    }

    dequeue(): Block | undefined {
        return this.blocks.shift();
    }

    isEmpty(): boolean {
        return this.blocks.length === 0;
    }
}

export type Block = {
    heat: number;
    r: number;
    c: number;
    dr: number;
    dc: number;
    n: number;
};
