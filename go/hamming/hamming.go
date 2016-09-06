package hamming

import (
	"errors"
)

const testVersion = 4

func Distance(a, b string) (int, error) {
	n := 0

	if len(a) != len(b) {
		return -1, errors.New("String lengths differ.")
	}

	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			n += 1
		}
	}

	return n, nil
}
