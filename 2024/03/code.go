package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var pattern *regexp.Regexp = regexp.MustCompile(`mul\((\d{1,3}),(\d{1,3})\)`)

type Pair struct {
	A int
	B int
}

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

	lines := []string{}

	for scanner.Scan() {
		line := scanner.Text()

		lines = append(lines, line)
	}

	if step == "1" {
		process_1(lines)
	} else if step == "2" {
		process_2(lines)
	} else {
		fmt.Println("Must pick a step")
	}
}

func process_1(lines []string) {
	mults := []Pair{}

	for _, line := range lines {
		matches := pattern.FindAllStringSubmatch(line, -1)

		if matches == nil {
			return
		}

		for _, match := range matches {
			a, err := strconv.Atoi(match[1])
			if err != nil {
				panic(err)
			}
			b, err := strconv.Atoi(match[2])
			if err != nil {
				panic(err)
			}

			mults = append(mults, Pair{a, b})
		}
	}

	sum := 0

	for _, mult := range mults {
		sum = sum + (mult.A * mult.B)
	}

	fmt.Println(sum)
}

//var pattern *regexp.Regexp = regexp.MustCompile(`mul\((\d{1,3}),(\d{1,3})\)`)

var pattern2 *regexp.Regexp = regexp.MustCompile(`(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don\'t\(\))`)

func process_2(lines []string) {
	input := strings.Join(lines, "")
	parsing := true
	total := 0

	matches := pattern2.FindAllStringSubmatch(input, -1)

	for i, match := range matches {
		fmt.Printf("%d: %v\n", i, match)
		if match[1] == "do()" {
			parsing = true
		} else if match[1] == "don't()" {
			parsing = false
		} else if parsing == true {
			a, err1 := strconv.Atoi(match[2])
			b, err2 := strconv.Atoi(match[3])
			if err1 != nil {
				panic(err1)
			}
			if err2 != nil {
				panic(err2)
			}

			total += a * b
		}
	}

	fmt.Println(total)
}
