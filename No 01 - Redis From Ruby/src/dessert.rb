require "redis"

redis=Redis.new(host:"localhost")

# Hash içerisine birkaç player örneği ekliyoruz
redis.hmset("player:1","fullname","Baz Layt Yiır","country","moon","score",339)
redis.hmset("player:2","fullname","Mega maynd","country","mars","score",317)
redis.hmset("player:5","fullname","Payn payn laki lu","country","wild west","score",405)
redis.hmset("player:3","fullname","Aeyrın Vaykovski","country","poland","score",322)
redis.hmset("player:4","fullname","Bileyd Rut","country","saturn","score",185)

# hgetall metoduna verdiğimiz parametre ile player:3 veri setini çekip ekrana bastırıyoruz
# Select'in where koşulu gibi düşünelim
player3=redis.hgetall("player:3")
puts "#{player3}\n\n"

# hmget ile player:2 key içeriğinden sadece country ve fullname değerlerini yazdırıyoruz
puts "#{redis.hmget('player:2','country','fullname')}\n\n"

# bir sorted list oluşturuyoruz
# listeyi yukarudaki hash üzerinden oluşturmak için player:* desenine uygun olan key içeriklerini çekmemiz lazım
# scan_each metodunun match parametresi bunu sağlıyor
redis.scan_each(:match=>"player:*"){|key|
    currentScore=redis.hmget(key,"score") #hmget metoduna iterasyondaki key değerini verip score bilgisini yakalıyoruz
    redis.zadd("score_list",currentScore,key) # score bilgisini o anki key ile ilişkilendirerek sorted list nesnesine ekliyoruz
}

# sorted list için skoru 330 puanın altına olanların listesini çektik (Select'in where koşulu gibi düşünelim)
scores_under_330=redis.zrangebyscore("score_list",0,330)

puts "Skoru 0 ile 330 arasında olanlar\n\n"
# bulduğumuz listede dolaşıp sadece fullname bilgilerini ekrana bastırdık
scores_under_330.each do |plyr|
    info=redis.hgetall(plyr)
    puts "#{info['fullname']} için güncel skor değeri #{info['score']}"
end