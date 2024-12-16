package main

import "fmt"

func merge(input []int, low int, mid int, high int) {
	// fmt.Println(">>> Left", input[low:mid])
	// fmt.Println(">>> Right", input[mid:high])
	var res = []int{}

	// println(low, mid, high)
	var l, r = low, mid

	for l < mid && r < high {
		if input[l] <= input[r] {
			res = append(res, input[l])
			l = l + 1
		} else {
			res = append(res, input[r])
			r = r + 1
		}
	}

	for l < mid {
		res = append(res, input[l])
		l = l + 1
	}
	for r < high {
		res = append(res, input[r])
		r = r + 1
	}

	var c = 0
	for i := low; i < high; i++ {
		input[i] = res[c]
		c++
	}
	// fmt.Println("::", res)
	// fmt.Println("::", input)
}

func merge_sort(low int, high int, input []int) {
	var mid = (low + high) / 2

	// fmt.Println("For input", input[low:high])
	// fmt.Println("Left", input[low:mid])
	// fmt.Println("Right", input[mid:high])

	if mid == low {
		return
	}

	merge_sort(low, mid, input)
	merge_sort(mid, high, input)

	merge(input, low, mid, high)
}

func main() {
	var input = []int{1, -3, 2, 1, 1, 5, 0, 4, 6, 0, 0, 0}
	merge_sort(0, len(input), input)
	fmt.Println(input)
}
