package com.learning.quoteworldweb.model;

/*
    Model sınıfımız. Yani Entity nesnemiz.
*/
import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.GenerationType;
import javax.persistence.Id;
import javax.persistence.Table;

@Entity
@Table(name="categories") // Veritabanındaki categories tablosunu işaret ettiğini belirtiyoruz
public class Category{

    // Tablodaki otomatik artan Identity alanımız

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    private String title;
    private Integer quotecount;

    // Varsayılan yapıcı metodumuz
    public Category()
    {
    }

    // Parametrik yapıcı metodumuz
    public Category(Long id,String title,Integer quotecount)
    {
        this.id=id;
        this.title=title;
        this.quotecount=quotecount;
    }

    public Long getId()
    {
        return this.id;
    }

    public String getTitle()
    {
        return this.title;
    }

    public void setTitle(String value)
    {
        this.title=value;
    }

    public Integer getQuotecount()
    {
        return this.quotecount;
    }

    public void setQuotecount(Integer value)
    {
        this.quotecount=value;
    }
}