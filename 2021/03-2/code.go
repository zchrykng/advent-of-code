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

	lines := []string{}

	scanner := bufio.NewScanner(file)

	t := MakeTree(12)

	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)
		t.AddLeaf(line, 0)
	}

	gamma, _ := t.GetGammaInt()
	epsilon, _ := t.GetEpsilonInt()
	oxygen, _ := t.GetOxygenRatingInt()
	CO2, _ := t.GetCO2RatingInt()
	fmt.Println("  Gamma:", t.GetGammaString(), gamma)
	fmt.Println("Epsilon:", t.GetEpsilonString(), epsilon)
	fmt.Println(" Oxygen:", t.GetOxygenRatingString(), oxygen)
	fmt.Println("    CO2:", t.GetCO2RatingString(), CO2)
	fmt.Println(" Answer:", oxygen*CO2)
}

func (t *Tree) GetGammaString() string {
	tmp := ""
	counts := t.GetCounts()

	for i := range counts {
		tmp = tmp + GetMostCommon(counts, i)
	}

	return tmp
}

func (t *Tree) GetEpsilonString() string {
	tmp := ""
	counts := t.GetCounts()

	for i := range counts {
		tmp = tmp + GetLeastCommon(counts, i)
	}

	return tmp
}

func (t *Tree) GetOxygenRatingString() string {
	workingtree := t
	depth := 0

	for len(workingtree.leaves) != 1 {
		counts := workingtree.GetCounts()

		if counts[depth].zero > counts[depth].one {
			workingtree = workingtree.zero
		} else if counts[depth].zero < counts[depth].one {
			workingtree = workingtree.one
		} else {
			workingtree = workingtree.one
		}

		depth = depth + 1
	}

	return workingtree.leaves[0]
}

func (t *Tree) GetCO2RatingString() string {
	workingtree := t
	depth := 0

	for len(workingtree.leaves) != 1 {
		counts := workingtree.GetCounts()

		if counts[depth].one > counts[depth].zero {
			workingtree = workingtree.zero
		} else if counts[depth].one < counts[depth].zero {
			workingtree = workingtree.one
		} else {
			workingtree = workingtree.zero
		}

		depth = depth + 1
	}

	return workingtree.leaves[0]
}

func (t *Tree) GetOxygenRatingInt() (int64, error) {
	return strconv.ParseInt(t.GetOxygenRatingString(), 2, 64)
}

func (t *Tree) GetCO2RatingInt() (int64, error) {
	return strconv.ParseInt(t.GetCO2RatingString(), 2, 64)
}

func (t *Tree) GetGammaInt() (int64, error) {
	return strconv.ParseInt(t.GetGammaString(), 2, 64)
}

func (t *Tree) GetEpsilonInt() (int64, error) {
	return strconv.ParseInt(t.GetEpsilonString(), 2, 64)
}

type Tree struct {
	zero   *Tree
	one    *Tree
	leaves []string
}

func MakeTree(depth int) *Tree {
	t := &Tree{}

	t.leaves = []string{}

	if depth > 0 {
		t.zero = MakeTree(depth - 1)
		t.one = MakeTree(depth - 1)
	}

	return t
}

func (t *Tree) AddLeaves(lines []string) {
	for _, l := range lines {
		t.AddLeaf(l, 0)
	}
}

func (t *Tree) AddLeaf(l string, depth int) {
	t.leaves = append(t.leaves, l)

	if len(l) > depth {
		if l[depth] == '0' {
			t.zero.AddLeaf(l, depth+1)
		} else if l[depth] == '1' {
			t.one.AddLeaf(l, depth+1)
		}
	}
}

func (t *Tree) FilterTree(search string) *Tree {
	tmp := t

	for _, c := range search {
		if c == '0' {
			tmp = tmp.zero
		} else if c == '1' {
			tmp = tmp.one
		}
	}

	return tmp
}

func (t *Tree) MatchingLeaves(search string) []string {
	return t.FilterTree(search).leaves
}

func (t *Tree) GetCounts() []count {
	counts := make([]count, 12)

	for _, l := range t.leaves {
		values := strings.Split(l, "")

		for i, v := range values {
			if v == "0" {
				counts[i].zero = counts[i].zero + 1
			}

			if v == "1" {
				counts[i].one = counts[i].one + 1
			}
		}
	}

	return counts
}

func GetMostCommon(counts []count, digit int) string {
	if counts[digit].one > counts[digit].zero {
		return "1"
	} else if counts[digit].one < counts[digit].zero {
		return "0"
	}
	return "1"
}

func GetLeastCommon(counts []count, digit int) string {
	if counts[digit].one < counts[digit].zero {
		return "1"
	} else if counts[digit].one > counts[digit].zero {
		return "0"
	}
	return "0"
}
