package main

import (
	"fmt"
	"github.com/natealcedo/monkey/repl"
	"os"
	"os/user"
)

func main() {
	usr, err := user.Current()
	if err != nil {
		panic(err)
	}
	fmt.Printf("Hello %s! This is the Monkey programming language!\n", usr.Username)
	fmt.Printf("Feel free to type out in commands\n")
	repl.Start(os.Stdin, os.Stdout)

}
