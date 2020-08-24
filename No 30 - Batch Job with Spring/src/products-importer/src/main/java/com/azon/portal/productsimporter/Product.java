package com.azon.portal.productsimporter;

public class Product {
    private String name;
    private Double unitPrice;
    private Integer quantity;

    // Default Constructor
    public Product() {
    }

    // Overload Construct üzerinden alanları doldurabiliriz
    public Product(String name, Double unitPrice, Integer quantity) {
        this.name = name;
        this.unitPrice = unitPrice;
        this.quantity = quantity;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getName() {
        return this.name;
    }

    public void setUnitPrice(Double unitPrice) {
        this.unitPrice = unitPrice;
    }

    public Double getUnitPrice() {
        return this.unitPrice;
    }

    public void setQuantity(Integer quantity) {
        this.quantity = quantity;
    }

    public Integer getQuantity() {
        return this.quantity;
    }

    @Override
    public String toString() {
        return name + " (" + unitPrice.toString() + ") - " + quantity.toString() + " adet";
    }
}