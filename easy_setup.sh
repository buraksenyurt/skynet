#!/bin/bash
echo ===============KOLAYCA KUR===================
echo "Klasör adı ne olacak?(Boşlukta koyabilirsin)"
echo =============================================

read folderName

mkdir "$folderName"
echo "$folderName isimli klasör oluşturuldu."
cd "$folderName"

echo "src, assets klasörleri ile Readme oluşturuluyor"
mkdir src
mkdir assets
touch README.md
echo "Kontrol et ;)"

echo =============================================
ls
echo =============================================

cd ..
echo "Code ile ilgili klasörü açıyoruz"
code .
