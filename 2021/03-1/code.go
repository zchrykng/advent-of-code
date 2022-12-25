package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type count struct {
	one  int
	zero int
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	defer file.Close()

	counts := make([]count, 12)

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		values := strings.Split(scanner.Text(), "")

		for i, v := range values {
			if v == "0" {
				counts[i].zero = counts[i].zero + 1
			}

			if v == "1" {
				counts[i].one = counts[i].one + 1
			}
		}
	}

	gamma := ""
	epsilon := ""

	for _, v := range counts {
		if v.one > v.zero {
			gamma = gamma + "1"
			epsilon = epsilon + "0"
		}
		if v.one < v.zero {
			gamma = gamma + "0"
			epsilon = epsilon + "1"
		}
	}

	gammaint, _ := strconv.ParseInt(gamma, 2, 64)
	epsilonint, _ := strconv.ParseInt(epsilon, 2, 64)

	fmt.Println(gammaint * epsilonint)
}
