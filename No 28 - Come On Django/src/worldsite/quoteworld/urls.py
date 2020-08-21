# Burası quoteworld uygulamasının view'ları için map tanımlamalarını yaptığımız yer
from django.urls import path

# ne kadar action varsa kullanabilmek için views'u kullanacağımızı söyledik
from . import views

urlpatterns = [
    # / yani root adrese gelen talepler index isimli view fonksiyonuna yönlenecek
    path('', views.index, name='index'),
    # quoteworld/category/5 benzeri bir talep gelirse, views modülünde quotesByCategory fonksiyonu tarafından ele alınacak
    path('/category/<int:category_id>/',
         views.quotesByCategory, name='quotelist'),
    # Yeni bir quote eklemek için kullanılan yol
    # Views içerisinde addQuote action'ının çağırır
    path('/addQuote/', views.addQuote, name='addQuote')
]


# Buradaki url map atamaları yeterli değil. Ayrıca worldsite içerisindeki urls.py dosyasında da web uygulamasına ait yönlendirme için bu modülün kullanılacağını bildirmemiz lazım
