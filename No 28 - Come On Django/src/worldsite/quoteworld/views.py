# Burası view modülümüz. Bir view modülünün görevi HTTP response döndürmektir.
# Tabii istisnai durumlarda geriye HTTP 400,404 gibi durum kodları da dönülebilir


# from django.http import HttpResponse # HttpResponse nesnesini alıyoruz
# HttpResponse nesnesini kullanmamıza gerek bırakmayacak
from django.shortcuts import render
# template klasöründeki index.html'i yüklemekte kullanacağız
from django.template import loader

from .models import Category

# Artık geriye kategori listesini linkler halinde gösterecek olan index.html içeriğini basıyoruz


def index(request):
    # isme göre sıralayarak tüm kategorileri çektik
    allCategories = Category.objects.order_by('title')
    # index.html sayfasını template olarak aldık
    # indexTemplate = loader.get_template('quoteworld/index.html') # render nesnesini kullandığımız için gerek kalmadı
    # index.html içerisinde category_list isimli bir değişken kullanıyoruz.
    # bunu context üzerinden template nesnesine taşıyabiliriz
    context = {'category_list': allCategories}
    # return HttpResonse(indexTemplate.render(context, request)) # aşağıdaki render kullanımı nedeniyle gerek kalmadı
    # gelen talebi al, index.html sayfasını context içeriği ile birlikte yorumlayıp geriye döndür diyor
    return render(request, 'quoteworld/index.html', context)
