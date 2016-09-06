package gigasecond

import (
	"math"
	"time"
)

const testVersion = 4

func AddGigasecond(start time.Time) time.Time {
	giga := math.Pow(10, 9)
	gigasecond := time.Duration(giga) * time.Second

	return start.Add(gigasecond)
}
