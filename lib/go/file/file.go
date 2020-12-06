package file

import (
	"bufio"
	"log"
	"os"
)

func ReadFile(filename string, handler func(index int, line string) error) error {
	file, err := os.Open(filename)
	if err != nil {
		return err
	}
	defer func() {
		err := file.Close()
		if err != nil {
			log.Printf("error occured on file close: %v", err)
		}
	}()

	scanner := bufio.NewScanner(file)
	for i := 0; scanner.Scan(); i++ {
		err := handler(i, scanner.Text())
		if err != nil {
			return err
		}
	}

	if err := scanner.Err(); err != nil {
		return err
	}

	return nil
}
