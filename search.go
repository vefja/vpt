package main

import (
	"bytes"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
)

func visit(files *[]string) filepath.WalkFunc {
	return func(path string, info os.FileInfo, err error) error {
		if err != nil {
			log.Fatal(err)
		}
		*files = append(*files, path)
		return nil
	}
}

func reverse(s string) string {

	words := strings.Fields(s) // tokenize each words from input string
	totalLength := len(words)

	// reverse the order(no sorting!)
	for i, j := 0, totalLength-1; i < j; i, j = i+1, j-1 {
		words[i], words[j] = words[j], words[i]
	}

	// return the reversed words
	return strings.Join(words, " ")
}

func main() {
	var files []string
	var searched string
	var success bool
	var hold_file string

	if len(os.Args[1]) == 0 {
		log.Fatal()
	}

	searched = os.Args[1]

	root := "/etc/elements/repos/"
	err := filepath.Walk(root, visit(&files))
	if err != nil {
		panic(err)
	}
	for _, file := range files {
		if strings.Contains(file, searched) {
			file := []byte(file)
			file = bytes.Replace(file, []byte("/etc/elements/repos/"), []byte(""), 1)
			file = bytes.Replace(file, []byte("/"), []byte(" / "), 1)
			success = true
			hold_file = reverse(string(file))
			file = []byte(hold_file)
			file = bytes.Replace(file, []byte(" / "), []byte("/"), 1)
			//pkg = string(file)
			fmt.Print(string(file))
			fmt.Print(" found.\n")
		}

	}
	if success {
	} else {
		log.Fatal()
	}
}
