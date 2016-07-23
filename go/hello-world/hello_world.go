package hello

const testVersion = 2

func HelloWorld(str string) string {
	if str == "" {
		return "Hello, World!"
	} else {
		return "Hello, " + str + "!"
	}
}
