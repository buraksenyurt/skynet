from django import forms

from .models import Quote, Category

'''
    forms kullanımı işimizi epey kolaylaştıracak.
    Şöyleki;
    QuotePostForm html tarafında bağlanmış olduğu modeldeki içeriği otomatik olarak gösterir.
    İşin güzel yanı, Quote'un bağlı olduğu Category ile aradaki ilişkiyi otomatik olarak algılar ve
    html tarafında bunun için hazır bir dropdown kullanır.

'''
class QuotePostForm(forms.ModelForm):

    class Meta:
        model = Quote # Form'un ilişkili olduğu modeli belirttik
         # fields = ('content', 'author', 'popularity_point', 'category')
        fields = ('__all__') # modeldeki tüm alanların form field olarak kullanılacağını ifade ettik. Yukarıdaki gibi sadece istediğimiz alanların kullanılmasını da sağlayabiliriz
