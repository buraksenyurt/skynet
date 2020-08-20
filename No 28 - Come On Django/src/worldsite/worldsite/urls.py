from django.contrib import admin
from django.urls import include, path #include'u aşağıda kullanabilmek için ayrıca ekledik

urlpatterns = [
    path('admin/', admin.site.urls),
    path('quoteworld',include('quoteworld.urls')) # quoteworld uygulamasına ait url yönlendirmeleri için hangi modülün kullanılacağını belirttik.
]
