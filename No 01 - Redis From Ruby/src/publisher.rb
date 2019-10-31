require 'redis'

puts 'Maç bilgileri gönderiliyor...'
redis=Redis.new(host:"localhost") #Redis sunucusuna bağlan

redis.publish 'game-info-101','Harden üç sayılık basket. Skor 92-92' # bir mesaj fırlat
redis.publish 'game-info-102','Furkan top çalma, hızlı hücum ve basket. Skor 2-0'
sleep 16
redis.publish 'game-info-102','Joel Embit blog. Skor 2-0'
sleep 14 # 14 saniye bekle
redis.publish 'game-info-101','Donçiç harika bir assist yapıyor ve Maksi Kleber smacı vuruyor. Skor 92-94'
sleep 22 # 22 saniye bekle
redis.publish 'game-info-101','Dallas son bir molaya gidiyor'
sleep 60 # 1 dakika bekle
redis.publish 'game-info-101','exit' # aboneler, bu kanal için aboneliklerini sonlandırabilir
redis.publish 'game-info-102','exit'

puts 'Program sonu'