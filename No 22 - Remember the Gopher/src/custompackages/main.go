package main

import (
	"custompackages/einstein" // Bu örnek için yazdığımız örnek bir paket
	"fmt"
)

func main() {
	var a, b int = 1, 6
	result := einstein.Sum(a, b)
	fmt.Println("Toplam ", result)
}

func init() {
	fmt.Println("Main'e ait init fonksiyonu")
}
