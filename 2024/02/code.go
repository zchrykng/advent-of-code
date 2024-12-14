package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const (
	safety_limit_low  = 1
	safety_limit_high = 3
)

func main() {
	step := os.Args[1]
	fileName := os.Args[2]
	if _, err := os.Stat(fileName); errors.Is(err, os.ErrNotExist) {
		panic(err)
	}

	file, err := os.Open(fileName)
	defer file.Close()
	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(file)

	reports := [][]int{}

	for scanner.Scan() {
		line := scanner.Text()

		items := strings.Split(line, " ")

		report := []int{}
		for _, item := range items {
			value, err := strconv.Atoi(item)
			if err != nil {
				panic(err)
			}
			report = append(report, value)
		}

		reports = append(reports, report)
	}

	if step == "1" {
		process_1(reports)
	} else if step == "2" {
		process_2(reports)
	} else {
		fmt.Println("Must pick a step")
	}
}

func report_str(report []int) string {
	tmp := []string{}
	for _, v := range report {
		tmp = append(tmp, fmt.Sprintf("%d", v))
	}
	return strings.Join(tmp, " ")
}

func process_1(reports [][]int) {
	safe_count := 0

	for i, report := range reports {
		//safe, reason := is_safes(report)
		safe := is_safe2(report)
		//fmt.Printf("Report: %d - %v - %s - %s\n", i, safe, report_str(report), reason)
		fmt.Printf("Report: %d - %v - %s\n", i, safe, report_str(report))
		if safe {
			safe_count += 1
		}
	}

	fmt.Println(safe_count)
}

func process_2(reports [][]int) {
	safe_count := 0

	for i, report := range reports {
		//safe, reason := is_safes(report)
		safe := is_safe2(report)
		if !safe {
			fmt.Println(report)
			for i := range report {
				subSlice := append([]int{}, report...)
				tmp_report := append(subSlice[:i], subSlice[i+1:]...)
				fmt.Println(tmp_report)
				subSafe := is_safe2(tmp_report)
				if subSafe {
					safe = subSafe
					break
				}
			}
		}
		//fmt.Printf("Report: %d - %v - %s - %s\n", i, safe, report_str(report), reason)
		fmt.Printf("Report: %d - %v - %s\n", i, safe, report_str(report))
		if safe {
			safe_count += 1
		}
	}

	fmt.Println(safe_count)
}

func is_safe2(report []int) bool {
	diffs := []int{}

	for i := range report {
		if i == 0 {
			continue
		}

		diff := report[i] - report[i-1]

		if diff == 0 {
			return false
		}

		diffs = append(diffs, diff)
	}

	for i, d := range diffs {
		if int_abs(d) > safety_limit_high || int_abs(d) < safety_limit_low {
			return false
		}

		if i != 0 {
			if signbit_int(diffs[i]) != signbit_int(diffs[i-1]) {
				return false
			}
		}
	}

	return true

}

func signbit_int(a int) int {
	if a > 0 {
		return 1
	}
	if a < 0 {
		return -1
	}
	return 0
}

func is_safe(report []int) (bool, string) {
	var previous int
	var previous_diff int
	for i, value := range report {
		if i == 0 {
			previous = value
			continue
		}

		if i == 1 {
			previous_diff = value - previous
			previous = value
			continue
		}

		diff := value - previous

		if diff == 0 {
			return false, "no change"
		} else if int_abs(diff) < safety_limit_low {
			return false, "outside safety - low"
		} else if int_abs(diff) > safety_limit_high {
			return false, "outside safety - high"
		} else if previous_diff < 0 && diff > 0 {
			return false, "direction change - decreasing to increasing"
		} else if previous_diff > 0 && diff < 0 {
			return false, "mismatch change - increasing to decreasing"
		}

		previous = value
		previous_diff = diff
	}

	return true, "safe"
}

func int_abs(value int) int {
	if value < 0 {
		return value * -1
	} else {
		return value
	}
}
