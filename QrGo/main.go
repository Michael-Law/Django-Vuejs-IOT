package main

import (
	"fmt"

	qrcode "github.com/skip2/go-qrcode"
)

func main() {
	err := qrcode.WriteFile("Rose Hill", qrcode.Medium, 256, "qr.png")
	if err != nil {
		fmt.Println("write error")
	}
}
