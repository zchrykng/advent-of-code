package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
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
		process_1()
	} else if step == "2" {
		process_2()
	} else {
		fmt.Println("Must pick a step")
	}
}

func process_1() {

}

func process_2() {

}
