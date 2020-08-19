package com.skynet.gamesworldapi; // Hangi pakete dahil

import org.springframework.data.annotation.Id;

/*
    MongoDb tarafındaki veriyi işaret eden eş nesne.
    Bir POJO (Plain Old Java Object)
    Lakin, C# Auto Property'lerin gözünü seveyim :D
*/
public class Player {

    @Id
    private String id; // import edilen paket üstünden kullandığımız Field. MongoDB ObjectId için
                       // kullanıyoruz. @Id niteliği ile bunu ifade etmiş olduk

    private String _nickName;
    private Integer _level;
    private Boolean _isActive;

    public String getNickName() {
        return _nickName;
    }

    public void setNickName(String nickName) {
        _nickName = nickName;
    }

    public Integer getLevel() {
        return _level;
    }

    public void setLevel(Integer level) {
        _level = level;
    }

    public Boolean getIsActive() {
        return _isActive;
    }

    public void setIsActive(Boolean isActive) {
        _isActive = isActive;
    }
}