package com.bemewe.services;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.ws.server.endpoint.annotation.Endpoint;
import org.springframework.ws.server.endpoint.annotation.PayloadRoot;
import org.springframework.ws.server.endpoint.annotation.RequestPayload;
import org.springframework.ws.server.endpoint.annotation.ResponsePayload;

/*
    Endpoint annotation ile sınıfın bir servis endpoint olarak çalışacağını belirttik.
    (Kime belirttik derseniz cevap Spring WS modülü)
    Autowired ise constructor'ın Spring'in DI mekanizmasına otomatik olarak bağladı.
*/
@Endpoint
public class CustomerEndpoint {

    private CustomerRepository _repository;

    @Autowired
    public CustomerEndpoint(CustomerRepository repository) {
        _repository = repository;
    }

    /*
     * Bu Endpoint üzerinden sunduğumuz operasyonu karşılayacak olan metot. 
     * Gelen XML paketindeki hangi namespace ve operasyon bilgisine cevaben çalışacağını @PayloadRoot ile belirttik.
     * 
     * Gelen XML paketi metodun request parametresine bağlanacak. Bunu @RequestPayload ile bildiriyoruz.
     * 
     * Tam tersi metod çıktısınında cevaben dönecek XML paketinde map edileceğiniz @ResponsePayload ile belirtmekteyiz.
     * 
     * setUsageSummary'nin UsageSummary'yi otomatik olarak nasıl alabildiğini merak etmiş olabilirsiniz. 
     * Plug-In ile üretilen GetCustomerUsageResponse sınıfı bu bilgiyi XSD içeriğinden alıp gerekli sınıf üretimini gerçekleştirdi.
     */
    @PayloadRoot(namespace = "http://bemewe.com/services", localPart = "getCustomerUsageRequest")
    @ResponsePayload
    public GetCustomerUsageResponse getUsageSummary(@RequestPayload GetCustomerUsageRequest request) {
        UsageSummary summary = _repository.GetSummaryByRegion(request.getRegion(), request.getSize());
        GetCustomerUsageResponse response = new GetCustomerUsageResponse();
        response.setUsageSummary(summary);
        return response;
    }
}