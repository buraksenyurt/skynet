package com.azon.cargotracer;

/*
    .Net Core tarafında RabbitMQ kuyruğuna attığımız Package sınıfı JSON formatta serileşerek yollanıyor.
    Onun Java tarafındaki izdüşümü kabul edeceğimiz sınıfı aşağıdaki gibi tanımlayabiliriz.
*/
public class PackageInfo {

    public int SerialNo;
    public String State;
    public double Weight;
    public String Time;

    public int getSerialNo() {
        return SerialNo;
    }

    public void setSerialNo(int value) {
        SerialNo = value;
    }

    public String getState() {
        return State;
    }

    public void setState(String value) {
        State = value;
    }

    public double getWeight() {
        return SerialNo;
    }

    public void setWeight(double value) {
        Weight = value;
    }

    public String getTime() {
        return Time;
    }

    public void setTime(String value) {
        Time = value;
    }
}
