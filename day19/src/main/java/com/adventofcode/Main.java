package com.adventofcode;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Main {
	public static void main(String[] args) throws IOException {
		System.out.printf("Part 1: %d\n", part1("input"));
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
										"<".equals(m.group(2)) ? Rule.Operation.LT : Rule.Operation.GT,
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

	public static int part1(String filename) throws IOException {
		List<String> input = Files.readAllLines(Path.of(filename));

		int s = input.indexOf("");
		Map<String, Rules> ruleMap = parseRules(input.subList(0, s));
		List<Map<String, Integer>> parts = parseParts(input.subList(s + 1, input.size()));


		return parts.stream()
				.map(part -> {
					String label = "in";

					while (!List.of("A", "R").contains(label)) {
						Rules rules = ruleMap.get(label);

						Optional<Rule> firstMatch = rules.getRules().stream()
								.filter(r -> checkCondition(r, part))
								.findFirst();
						label = firstMatch.map(Rule::getLabel).orElseGet(rules::getLabel);
					}
					return label.equals("A") ? part.values().stream().reduce(Integer::sum).orElse(0) : 0;
				})
				.reduce(Integer::sum)
				.orElse(0);
	}

	public static boolean checkCondition(Rule r, Map<String, Integer> part) {
		int v = part.get(r.getPart());
		return (r.getOp() == Rule.Operation.LT && v < r.getValue()) ||
				(r.getOp() == Rule.Operation.GT && v > r.getValue());
	}
}
