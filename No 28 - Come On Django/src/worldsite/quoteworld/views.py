# HttpResponse nesnesini alıyoruz
from django.http import HttpResponse

# ki / ile index sayfasına yönlendirildiğimizde geriye bir "Hello World" içeriği döndürelim
def index(request):
    return HttpResponse("<b>Index sayfası</b>")

