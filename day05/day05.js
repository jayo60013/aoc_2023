async function part1(filename) {
  const content = await Deno.readTextFile(filename);

  const [seeds_line, ...section] = content.split("\n\n");
  let seeds = seeds_line
    .split(": ")[1]
    .split(" ")
    .map((n) => parseInt(n));

  for (let i = 0; i < seeds.length; i++) {

    for (let j = 0; j < section.length; j++) {
      const [_, ...lines] = section[j].split("\n");

      for (let k = 0; k < lines.length; k++) {
        const [dest_start, source_start, range] =
          lines[k]
            .split(" ")
            .map((n) => parseInt(n));
        if (isNaN(dest_start)) continue;

        if (source_start <= seeds[i] && seeds[i] < source_start + range) {
          seeds[i] = (dest_start - source_start) + seeds[i];
          break;
        }
      }
    }
  }

  return Math.min(...seeds);
}

async function part2(filename) {
  const content = await Deno.readTextFile(filename);

  const [seeds_line, ...section] = content.split("\n\n");
  let seeds = seeds_line
    .split(": ")[1]
    .split(" ")
    .reduce(
      (acc, _, i, arr) => {
        if (i % 2 === 0) {
          acc.push([parseInt(arr[i]), parseInt(arr[i]) + parseInt(arr[i + 1])])
        }
        return acc;
      }, []);

  let found = false;


  for (let i = 0; i < section.length; i++) {
    const [_, ...lines] = section[i].split("\n");

    let new_seeds = [];
    while (seeds.length > 0) {
      const [start, end] = seeds.pop();

      for (let j = 0; j < lines.length; j++) {
        const [a, b, c] =
          lines[j]
            .split(" ")
            .map((n) => parseInt(n));
        if (isNaN(a)) new_seeds.push([start, end]);

        let os = Math.max(start, b);
        let oe = Math.min(end, b + c);

        if (os < oe) {
          new_seeds.push(
            [os - b + a,
            oe - b + a]
          );
          if (os > start) seeds.push([start, os]);
          if (end > oe) seeds.push([oe, end]);
          found = true;
          break;
        }
      }
      if (found) {
        new_seeds.push([start, end]);
        found = false;
      }
    }
    seeds = new_seeds;
    console.log(seeds);
  }
  console.log(seeds);
}

// console.log("Part 1:", await part1("sample.txt"));
// console.log("Part 2:", await part2("sample.txt"));
part2("sample.txt");
