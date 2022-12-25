package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	values := []int{}

	for scanner.Scan() {
		value, _ := strconv.Atoi(scanner.Text())

		values = append(values, value)
	}

	increases := 0
	last := -1
	for i := 0; i < len(values)-2; i++ {
		if i == 0 {
			last = values[i] + values[i+1] + values[i+2]
		}

		v := values[i] + values[i+1] + values[i+2]

		if last < v {
			increases = increases + 1
		}

		last = v
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println(increases)
}
