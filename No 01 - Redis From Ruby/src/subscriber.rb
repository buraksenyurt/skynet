require 'redis'

channelName=ARGV[0] # komut satırında kanal bilgisini al

redis=Redis.new(host:"localhost") #Redis sunucusuna bağlan

begin
    # game-info-101 isimli olaya abone ol
    redis.subscribe channelName do |on|

    on.subscribe do |channel,msg| # abonelik gerçekleşince çalışır
        puts "#{channel} kanalına abone olundu"
    end
    
    on.message do |channel, msg| # mesaj gelince çalışır
      puts "#{channel} -> #{msg}" # mesajı ekrana bastır
      redis.unsubscribe if msg=="exit" # eğer mesaj bilgisi exit olarak geldiyse aboneliği sonlandır
    end

    on.unsubscribe do |channel,msg| # abonelik sonlandığında çalışır
        puts "#{channel} kanalına abonelik sonlandırıldı"
    end
end
rescue redis::BaseConnectionError => err # bir bağlantı hatası sorunu olursa 3 saniye içinde tekra bağlanmaya çalışılır
    puts "#{err}, 3 saniye içinde tekrar deneyeceğim"
    sleep 3
    retry
end