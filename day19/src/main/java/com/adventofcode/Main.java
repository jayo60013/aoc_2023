package com.adventofcode;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Main {
	public static void main(String[] args) throws IOException {
		parseInput("sample.txt");
	}

	public static List<String> parseInput(String filename) throws IOException {
		List<String> input = Files.readAllLines(Path.of("sample.txt"));

		Map<String, Rules> ruleMap = new HashMap<>();
		List<List<Integer>> parts = new ArrayList<>();
		boolean parseRules = true;

		for (String line : input) {
			if (line.isEmpty()) {
				parseRules = false;
			} else if (parseRules) {
				String label = line.split("\\{")[0];

				String rulesString = line.split("\\{")[1];
				Rules rules = new Rules();
				for (String r : rulesString.split(",")) {
					Pattern p = Pattern.compile("(x|m|a|s)?(<|>)?(\\d+)?:?(\\w+),?");
					Matcher m = p.matcher(r);

					if (m.find()) {
						if (m.group(1) == null) {
							rules.setLabel(m.group(4));
						} else {
							rules.addRule(new Rule(
									m.group(1),
									(m.group(2) == "<") ? Rule.Operation.LT : Rule.Operation.GT,
									Integer.parseInt(m.group(3)),
									m.group(4)
							));
						}
					}
				}
				ruleMap.put(label, rules);
			} else {
				line = line.substring(1, line.length() - 1);
				Pattern p = Pattern.compile("(\\d+)+");
				Matcher m = p.matcher(line);

				List<Integer> part = new ArrayList<>();
				while (m.find()) {
					part.add(Integer.parseInt(m.group()));
				}
				parts.add(part);
			}
		}
		return input;
	}
}