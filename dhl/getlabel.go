package main

import (
	/*
		#cgo LDFLAGS: -L . -lshipmodule
		#include "./shipmodule.h"
	*/
	"C"
	"fmt"
)

//export GetLabel
func GetLabel(input string) {
	fmt.Printf("Getting label for %s\n", input)
	C.rust_expose(C.CString(input))
}

//export GetChannel
func GetChannel(input chan string) {
	input <- "Nice"
}

func main() {
	C.rust_expose(C.CString("Test"))
}
