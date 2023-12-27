package main

func isSubset(superset, subset []int) bool {
	for _, v := range subset {
		if !contains(superset, v) {
			return false
		}
	}
	return true
}

func contains(slice []int, n int) bool {
	for _, v := range slice {
		if v == n {
			return true
		}
	}
	return false
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a < b {
		return b
	}
	return a
}
