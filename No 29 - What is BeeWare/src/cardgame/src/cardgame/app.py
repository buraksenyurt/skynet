"""
A simple desktop application on Ubuntu
"""
import toga
from toga.style import Pack
from toga.style.pack import COLUMN, ROW


class CardGame(toga.App):

    def startup(self):

        # Bu ana form gibi bir şey. Toga uygulamalarında kutu koleksiyonları da söz konusu olabilir(Box Collection)
        # direction için verdiğim Column ile main_box'a eklenecek kontrollerin sütun formatında aşağıya doğru ineceğini söyledik
        main_box = toga.Box(style=Pack(direction=COLUMN))

        # Şimdi birkaç kontrol ekleyelim
        # Birkaç Label ekliyoruz ve onlara padding stilleri veriyoruz
        lblNickname = toga.Label(
            'Takma adını söyler misin?', style=Pack(padding=(0, 10)))  # Biraz padding ayarı yaptık
        lblColor = toga.Label('En sevdiğin renk?', style=Pack(padding=(0, 10)))
        lblLuckyNumber = toga.Label(
            'Şans numaran peki?', style=Pack(padding=(0, 10)))
        self.lblMyGuess = toga.Label('...', style=Pack(flex=1, padding=10))

        # Şimdi yukarıdaki sorular için birer Input kontrolü ekleyelim
        # Bunları sınıfın Instance Variable'ları olarak tanımlıyoruz ki erişmemiz kolay olsun.
        self.txtNickname = toga.TextInput(style=Pack(padding=5, flex=1))
        self.txtColor = toga.TextInput()
        self.txtLuckyNumber = toga.TextInput()

        # Şimdi yukarıdaki kontrolleri bir kutuya koyalım. (Box)
        boxIdentity = toga.Box(style=Pack(direction=COLUMN))
        boxIdentity.add(lblNickname)
        boxIdentity.add(self.txtNickname)
        boxIdentity.add(lblColor)
        boxIdentity.add(self.txtColor)
        boxIdentity.add(lblLuckyNumber)
        boxIdentity.add(self.txtLuckyNumber)

        # Sonra bu kontrolleri içeren kutuyu ana kutuya ekleyelim
        main_box.add(boxIdentity)

        # Bir tane de Button oluşturup ana kutuya ilave edelim
        # Button'a basıldığında da bntGuess_OnPress isimli olay metodu çalışacak
        btnGuess = toga.Button(
            'Tahmin Yap', on_press=self.btnGuess_OnPress, style=Pack(width=100, padding=10))
        main_box.add(btnGuess)

        main_box.add(self.lblMyGuess)

        # Title ile biraz oynadım. Keh keh keh :P
        self.main_window = toga.MainWindow(
            title=self.formal_name+" Version 1.0")
        self.main_window.content = main_box
        self.main_window.show()

        # Windows Forms tarafına ne kadar da benziyor
    def btnGuess_OnPress(self, widget):
        print('Belki loglama amaçlı kullanılabilir')
        # Burada validasyon yapmak gerekir mi? Yoksa kontrollerin validasyon için nitelikleri var mıdır?
        summary = "Merhaba {nick}. Favori rengin {color} ve şans numaran {number}".format(
            nick=self.txtNickname.value, color=self.txtColor.value, number=self.txtLuckyNumber.value)
        self.lblMyGuess.text = summary


'''
    Uygulama yüklendiğinde main metodu CardGame sınıfını örnekler
    Bunu bir Windows Forms sınıfının yüklenmesi olarak düşünüyorum ;)
'''


def main():
    return CardGame()
