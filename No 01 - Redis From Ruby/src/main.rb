require 'redis' # redis gem'ini kullanacağımızı belirttik

redis=Redis.new(host:"localhost")
#redis.ping()

redis.set("aloha",1001) # örnek key:value ekledik
word=redis.get("aloha") # eklediğimizi alıp 

puts word # ekrana bastırdık

# List kullanımına örnekler

redis.del('user_actions') # önce user_actions ve tutorial_list listelerini temizleyelim
redis.del('tutorial_list')

# Right Push
redis.rpush('user_actions','Naycıl login olmayı deniyor')
redis.rpush('user_actions','şifre hatalı girildi')
redis.rpush('user_actions','Naycıl giriş yaptı')
redis.rpush('user_actions','Naycıl alışveriş sepetine bakıyor')
redis.rpush('user_actions','Naycıl sepetten 19235123A kodlu ürünü çıkarttı')

p redis.lrange('user_actions',0,-1) # tüm listeyi ilk girişten itibaren getirir

redis.ltrim('user_actions',-1,-1) # son elemana kadar olan liste elemanlarını çıkarttık
puts ''
p redis.lrange('user_actions',0,-1)

puts ''

#Left Push
redis.lpush('tutorial_list','redis')
redis.lpush('tutorial_list','mongodb')
redis.lpush('tutorial_list','ruby on rails')
redis.lpush('tutorial_list','golang')

p redis.lrange('tutorial_list',0,-1)

# Set Kullanımına Örnekler

redis.del('cenifer-friends')
redis.del('melinda-friends')

redis.sadd('cenifer-friends','semuel')
redis.sadd('cenifer-friends','nora')
redis.sadd('cenifer-friends','mayki')
redis.sadd('cenifer-friends','lorel')
redis.sadd('cenifer-friends','bill')

redis.sadd('melinda-friends','mayki')
redis.sadd('melinda-friends','ozi')
redis.sadd('melinda-friends','bill')
redis.sadd('melinda-friends','törnır')
redis.sadd('melinda-friends','sementa')
redis.sadd('melinda-friends','kıris')

puts ''
p redis.smembers('cenifer-friends')
puts ''
p redis.smembers('melinda-friends')
puts ''
p redis.sinter('cenifer-friends','melinda-friends') # Yukarıdaki iki kümenin kesiştiği elemanları verir
puts ''
p redis.srandmember('melinda-friends') # her srandmember çağrısı kümeden rastgele bir elemanı döndürür
p redis.srandmember('melinda-friends')

# Sorted Set örnekleri

redis.del('best-players-of-the-week')
# haftanın oyuncularını ağırlık puanlarına göre ekledik
redis.zadd('best-players-of-the-week',32,'maykıl cordın')
redis.zadd('best-players-of-the-week',24,'skati pipın')
redis.zadd('best-players-of-the-week',32,'leri börd')
redis.zadd('best-players-of-the-week',21,'con staktın')

puts ''
puts redis.zrevrange('best-players-of-the-week',0,-1) # en yüksek skor puanından en küçüne doğru getirir (rev hecesine dikkat)
puts '' 
puts redis.zrevrange('best-players-of-the-week',0,0) # en iyi skora sahip olanı getirir (rev hecesine dikkat)
puts ''
puts redis.zrangebyscore('best-players-of-the-week',20,30) # skorları 20 ile 30 arasında olanlar

