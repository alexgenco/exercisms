package clock

import "fmt"

const testVersion = 4

type Clock struct {
	Hour   int
	Minute int
}

func New(hour, minute int) Clock {
	dayMinutes := 24 * 60
	minutes := (hour*60 + minute) % dayMinutes

	if minutes < 0 {
		minutes += dayMinutes
	}

	return Clock{Hour: minutes / 60, Minute: minutes % 60}
}

func (c Clock) String() string {
	return fmt.Sprintf("%02d:%02d", c.Hour, c.Minute)
}

func (c Clock) Add(minutes int) Clock {
	return New(c.Hour, c.Minute+minutes)
}
