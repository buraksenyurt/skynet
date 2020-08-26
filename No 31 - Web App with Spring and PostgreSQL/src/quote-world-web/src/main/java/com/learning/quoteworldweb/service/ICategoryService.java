package com.learning.quoteworldweb.service;

import java.util.List;
import com.learning.quoteworldweb.model.Category;

public interface ICategoryService {
    List<Category> getAll();

    Category getSingle(Long id);

    Long add(Category category);
}