package com.adventofcode;

import java.util.ArrayList;
import java.util.List;

public class Rules {
	private final List<Rule> rules;
	private String label;

	public Rules() {
		this.rules = new ArrayList<>();
	}

	public void addRule(Rule r) {
		this.rules.add(r);
	}

	public String getLabel() {
		return label;
	}

	public void setLabel(String label) {
		this.label = label;
	}
}
