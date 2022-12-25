package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	x := 0
	y := 0
	aim := 0

	for scanner.Scan() {
		values := strings.Split(scanner.Text(), " ")

		command := values[0]
		value, _ := strconv.Atoi(values[1])

		switch command {
		case "forward":
			x = x + value
			y = y + aim*value
		case "reverse":
			x = x - value
			y = y - aim*value
		case "down":
			aim = aim + value
		case "up":
			aim = aim - value
		}
	}

	fmt.Println(x * y)
}
