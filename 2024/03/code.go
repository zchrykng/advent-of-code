package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"regexp"
	"strconv"
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
		process_2()
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

func process_2() {

}
