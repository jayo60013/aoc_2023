package com.adventofcode;

public class Rule {

	private final String part;
	private final Operation op;
	private final Integer value;
	private final String label;

	public Rule(String part, Operation op, Integer value, String label) {
		this.part = part;
		this.op = op;
		this.value = value;
		this.label = label;
	}

	public enum Operation {
		LT,
		GT,
		NOOP,
	}
}