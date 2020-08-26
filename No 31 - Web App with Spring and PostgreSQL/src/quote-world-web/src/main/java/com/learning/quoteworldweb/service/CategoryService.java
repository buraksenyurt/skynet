package com.learning.quoteworldweb.service;

import java.util.List;
import com.learning.quoteworldweb.model.Category;
import com.learning.quoteworldweb.repository.CategoryRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

/*
    Tüm kategorileri ve bir id değerine göre tek kategoriyi döndüren operasyonları içeren servis sınıfımız.
    ICategoryService arayüzünü implemente ettiği için oradaki metodları ezmek zorundayız.
    findAll ve findById gibi fonksiyonlar CategoryRepository isimli repository sınıfı üzerinden kullanılmaktadır.
    add Metodunu ise yeni bir kategoriyi eklemek için kullanmaktayız.
*/

@Service
public class CategoryService implements ICategoryService {

    @Autowired
    private CategoryRepository repository; // Repository sınıfı enjekte ediliyor

    @Override
    public List<Category> getAll() {
        return (List<Category>) repository.findAll();
    }

    @Override
    public Category getSingle(Long id) {
        return repository.findById(id).get();
    }

    @Override
    public Long add(Category category){
        Long id=repository.save(category).getId();
        return id;
    }
}