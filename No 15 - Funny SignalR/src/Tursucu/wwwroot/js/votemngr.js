"use strict";

// Çalışma zamanında sayfadan gelen voting route kullanılarak bağlantı nesnesi oluşuyor
var connection = new signalR.HubConnectionBuilder().withUrl("/voting").build();

// bunları limoncu ve sirkecileri hesaplamak için kullanıyorum ancak önemli bir probleme neden oluyor?
var lemonPoints = 0;
var vinegardPoints = 0;

// Sunucu tarafından yayınlanan GetVote mesajını yakalıyor
connection.on("GetVoteMessage", function (user, choice) {

    // Güncel saat, kullanıcı ve yaptığı seçim bilgilerin
    // index sayfasının altındaki voices isimli listeye ekliyorum
    var today = new Date();
    var time = today.getHours() + ":" + today.getMinutes() + ":" + today.getSeconds();

    var voteMessage = "[" + time + "] " + user + " '" + choice + "' dedi.";
    var summary = document.getElementById("voices");
    var lineItem = document.createElement("li");
    lineItem.textContent = voteMessage;
    summary.insertBefore(lineItem, summary.childNodes[0]);

    // seçime göre puanları bir birim arttırıyorum
    if (choice == "limon")
        lemonPoints++;

    if (choice == "sirke")
        vinegardPoints++;

    // progressBar elementlerini yakalayıp güncel değerlere göre şekillendiriyorum
    var prgLemon = document.getElementById("prgLemon");
    prgLemon.innerHTML = "LİMON = " + lemonPoints;
    prgLemon.style.width = lemonPoints + "%";

    var prgVinegard = document.getElementById("prgVinegard");
    prgVinegard.innerHTML = "SİRKE = " + vinegardPoints;
    prgVinegard.style.width = vinegardPoints + "%";
});

// Sunucu ile aradaki bağlantı sağlandığında çalışıyor
connection.start().catch(function (err) {
    return console.error(err.toString());
});

// sayfadaki sendVote isimli button kontrolünün click olayını dinliyor
document.getElementById("sendVote").addEventListener("click", function (event) {

    // Kullanıcının girdiği bilgiyi alıyorum
    var user = document.getElementById("participantName").value;
    if (!user) {
        user = "[isimsiz]";
    }

    // RadioButton kontrollerinden hangisini seçtiğini buluyorum
    // neden name elementine voteOption diye ortak bir isim verdik
    // şimdi daha net oldu
    var choice = document.querySelector('input[name = voteOption]:checked').value;

    // Açık kanalı kullanarak PushVote isimli bir mesaj yayınlıyoruz
    // Parametre olarak kullanıcıyı ve yaptığı seçimi gönderiyoruz
    connection.invoke("PushVoteMessage", user, choice).catch(function (err) {
        return console.error(err.toString());
    });

    event.preventDefault();
});