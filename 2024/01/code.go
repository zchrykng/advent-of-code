package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"sort"
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

	list_a := []int{}
	list_b := []int{}

	for scanner.Scan() {
		line := scanner.Text()

		items := strings.Split(line, "   ")
		a, _ := strconv.Atoi(items[0])
		b, _ := strconv.Atoi(items[1])

		list_a = append(list_a, a)
		list_b = append(list_b, b)
	}

	if step == "1" {
		process_1(list_a, list_b)
	} else if step == "2" {
		process_2(list_a, list_b)
	} else {
		fmt.Println("Must pick a step")
	}

}

func process_1(list_a, list_b []int) {
	sort.Sort(sort.IntSlice(list_a))
	sort.Sort(sort.IntSlice(list_b))

	diff_sum := 0

	for i := range len(list_a) {
		a := list_a[i]
		b := list_b[i]
		diff := a - b
		if diff < 0 {
			diff = diff * -1
		}

		diff_sum += diff

		fmt.Printf("a: %d, b: %d, diff: %d, sum: %d\n", a, b, diff, diff_sum)
	}
}

func process_2(list_a, list_b []int) {
	counts := make(map[int]int)

	for _, v := range list_b {
		if _, prs := counts[v]; !prs {
			counts[v] = 0
		}
		counts[v] += 1
	}

	similarity := 0
	for _, v := range list_a {
		if count, prs := counts[v]; prs {
			similarity += v * count
			fmt.Printf("value: %d, count: %d, similarity: %d\n", v, count, similarity)
		}
	}

	fmt.Println(similarity)
}
