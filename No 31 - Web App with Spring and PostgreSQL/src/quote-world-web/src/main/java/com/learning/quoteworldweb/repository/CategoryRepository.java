package com.learning.quoteworldweb.repository;

import com.learning.quoteworldweb.model.Category;

import org.springframework.data.repository.CrudRepository;
import org.springframework.stereotype.Repository;

/*
    Standart CRUD operasyonlarını devraldığımız repository sınıfımız.
*/
@Repository
public interface CategoryRepository extends CrudRepository<Category, Long> {
}