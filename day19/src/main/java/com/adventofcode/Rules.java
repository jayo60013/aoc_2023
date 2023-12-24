package com.adventofcode;

import java.util.List;

public class Rules {
	private final List<Rule> rules;
	private final String fallback;

	public Rules(List<Rule> rs, String fallback) {
		this.rules = rs;
		this.fallback = fallback;
	}

	public List<Rule> getRules() {
		return rules;
	}

	public String getFallback() {
		return fallback;
	}
}