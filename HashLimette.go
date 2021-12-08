package main

import (
	"errors"
	"fmt"
	"io"
	"log"
	"os"
	"golang.org/x/crypto/bcrypt"
)

var (
	app       string
	email     string
	pw        string
	err       error
	passwords *os.File
	hash      []byte
)

func main() {
	inPut()
	genPass()

}

func inPut() {

	fmt.Println("Für welche Website oder auch app sind diese login daten? :")
	fmt.Scanln(&app)

	fmt.Println("Bitte Email oder Benutzer name eingeben : ")
	fmt.Scanln(&email)

}

func pasus() []byte {

	fmt.Println("Bitt jetzt das Passwort : ")
	fmt.Scanln(&pw)
	return []byte(pw)
}
func hashAndSalt(pw []byte) string {

	hash, err = bcrypt.GenerateFromPassword(pw, bcrypt.MinCost)
	if err != nil {
		log.Println(err)
		panic(err)
	}
	return string(hash)

}

func genPass() {
	if _, err = os.Stat("PW.txt"); errors.Is(err, os.ErrNotExist) {
		passwords, err = os.Create("PW.txt")
		_, err = io.WriteString(passwords, "Meine Daten Bank \n\n")
	}

	passwords, err = os.OpenFile("PW.txt", os.O_WRONLY|os.O_APPEND, os.ModePerm)
	if err != nil {
		panic(err)
	}
	defer passwords.Close()
	pw := pasus()
	hash := hashAndSalt(pw)
	_, err = io.WriteString(passwords, "\n"+app+" : "+email+" : "+hash)
}
