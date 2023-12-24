package com.adventofcode;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

record Rule(String part, String op, Integer value, String label) {
}

record Range(int start, int end) {
}

public class Main {
	private static Map<String, Rules> ruleMap;
	private static List<Map<String, Integer>> parts;

	public static void main(String[] args) throws IOException {
		List<String> input = Files.readAllLines(Path.of("input"));

		int s = input.indexOf("");
		ruleMap = parseRules(input.subList(0, s));
		parts = parseParts(input.subList(s + 1, input.size()));

		System.out.printf("Part 1: %d\n", part1());
		System.out.printf("Part 2: %d\n", part2());
	}

	public static long part2() {
		Map<String, Range> partRanges = new HashMap<>();
		for (String s : List.of("x", "m", "a", "s")) {
			partRanges.put(s, new Range(1, 4000));
		}
		return count(partRanges, "in");
	}

	public static long count(Map<String, Range> ranges, String label) {
		if (label.equals("R")) return 0;
		if (label.equals("A")) {
			return ranges.values().stream()
					.mapToLong(r -> r.end() - r.start() + 1)
					.reduce(1, (acc, length) -> acc * length);
		}

		Rules rules = ruleMap.get(label);

		long total = 0L;
		boolean needFallback = true;
		for (Rule rule : rules.getRules()) {
			Range accept;
			Range reject;
			if (rule.op().equals("<")) {
				accept = new Range(
						ranges.get(rule.part()).start(),
						rule.value() - 1
				);
				reject = new Range(
						rule.value(),
						ranges.get(rule.part()).end()
				);
			} else {
				accept = new Range(
						rule.value() + 1,
						ranges.get(rule.part()).end()
				);
				reject = new Range(
						ranges.get(rule.part()).start(),
						rule.value()
				);
			}
			if (accept.start() <= accept.end()) {
				Map<String, Range> newRanges = new HashMap<>(ranges);
				newRanges.put(rule.part(), accept);
				total += count(newRanges, rule.label());
			}
			if (reject.start() <= reject.end()) {
				ranges = new HashMap<>(ranges);
				ranges.put(rule.part(), reject);
			} else {
				needFallback = false;
				break;
			}
		}

		if (needFallback) {
			total += count(ranges, rules.getFallback());
		}
		return total;
	}

	public static Map<String, Rules> parseRules(List<String> input) {
		Map<String, Rules> ruleMap = new HashMap<>();
		Pattern labelPattern = Pattern.compile("^(\\w+)\\{(.*)\\}$");
		Pattern rulePattern = Pattern.compile("([xmas])?([<>])?(\\d+)?:?(\\w+)");

		for (String line : input) {
			Matcher ms = labelPattern.matcher(line);
			if (ms.find()) {
				String label = ms.group(1);
				String rest = ms.group(2);

				String[] rs = rest.split(",");
				List<Rule> ruleList =
						Arrays.stream(rs, 0, rs.length - 1)
								.map(rulePattern::matcher)
								.filter(Matcher::find)
								.map(m -> new Rule(
										m.group(1),
										m.group(2),
										Integer.parseInt(m.group(3)),
										m.group(4)
								))
								.toList();
				ruleMap.put(label, new Rules(ruleList, rs[rs.length - 1]));
			}
		}
		return ruleMap;
	}

	public static List<Map<String, Integer>> parseParts(List<String> input) {
		Pattern p = Pattern.compile("x=(\\d+),m=(\\d+),a=(\\d+),s=(\\d+)");

		return input.stream()
				.map(line -> line.substring(1, line.length() - 1))
				.map(p::matcher)
				.filter(Matcher::find)
				.map(m -> {
					Map<String, Integer> part = new HashMap<>();
					part.put("x", Integer.parseInt(m.group(1)));
					part.put("m", Integer.parseInt(m.group(2)));
					part.put("a", Integer.parseInt(m.group(3)));
					part.put("s", Integer.parseInt(m.group(4)));
					return part;
				})
				.toList();
	}

	public static int part1() {
		return parts.stream()
				.map(part -> {
					String label = "in";

					while (!List.of("A", "R").contains(label)) {
						Rules rules = ruleMap.get(label);

						Optional<Rule> firstMatch = rules.getRules().stream()
								.filter(r -> checkCondition(r, part))
								.findFirst();
						label = firstMatch.map(Rule::label).orElseGet(rules::getFallback);
					}
					return label.equals("A") ? part.values().stream().reduce(Integer::sum).orElse(0) : 0;
				})
				.reduce(Integer::sum)
				.orElse(0);
	}

	public static boolean checkCondition(Rule r, Map<String, Integer> part) {
		int v = part.get(r.part());
		return ("<".equals(r.op()) && v < r.value()) ||
				(">".equals(r.op()) && v > r.value());
	}
}