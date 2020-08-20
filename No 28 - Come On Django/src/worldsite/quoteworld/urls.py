# Burası quoteworld uygulamasının view'ları için map tanımlamalarını yaptığımız yer
from django.urls import path

# ne kadar action varsa kullanabilmek için views'u kullanacağımızı söyledik
from . import views

urlpatterns = [
    path('', views.index, name='index') # / yani root adrese gelen talepler index isimli view fonksiyonuna yönlenecek
]


# Bu atama yeterli değil. Ayrıca worldsite içerisindeki urls.py içerisine de web uygulamasına ait yönlendirme için bu modülün kullanılacağını bildirmemiz lazım