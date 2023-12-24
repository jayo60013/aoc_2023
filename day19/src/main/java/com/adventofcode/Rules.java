package com.adventofcode;

import java.util.List;

public class Rules {
	private final List<Rule> rules;
	private String label;

	public Rules(List<Rule> rs, String label) {
		this.rules = rs;
		this.label = label;
	}

	public void addRule(Rule r) {
		this.rules.add(r);
	}

	public void addAllRules(List<Rule> rs) {
		this.rules.addAll(rs);
	}

	public List<Rule> getRules() {
		return rules;
	}

	public String getLabel() {
		return label;
	}

	public void setLabel(String label) {
		this.label = label;
	}
}
