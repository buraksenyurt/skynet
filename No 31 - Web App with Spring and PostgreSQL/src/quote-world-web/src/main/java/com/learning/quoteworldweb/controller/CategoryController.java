package com.learning.quoteworldweb.controller;

import java.util.List;

import com.learning.quoteworldweb.model.Category;
import com.learning.quoteworldweb.service.ICategoryService;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PostMapping;

@Controller
public class CategoryController {
    @Autowired
    private ICategoryService categoryService; // Servis örneği enjekte ediliyor

    @GetMapping("/allCategories") // Path tanımı
    public String allCategories(Model model) {
        var result = (List<Category>) categoryService.getAll(); // Enjekte edilen servis üstünden tüm kategori listesi
                                                                // çekildi
        model.addAttribute("categoryList", result); // İlişkili model nesnesine attibute olarak ilgili liste eklendi
        return "allCategories"; // Model nesnesi, thymeleaf sayesinde allCategories.html dosyasına bağlanacak

        /*
         * Model üstünden categoryList değişkeni ile geriye döndürdüğümüz bir liste söz
         * konusu. allCategories.html dosyasında model'den gelen Category nesnelerini
         * HTML'e nasıl bağladığımıza dikkat edin.
         * 
         * Ayrıca yeni kategori eklemek için farklı bir view kullanılıyor. newCategory
         * path'ine gelen talepler newCategory.html şablonunu döndürmekte.
         * 
         * newCategory.html şablonundaki form HTTP Post ile yollandığındaysa PostMapping
         * niteliği ile işaretlenmiş olan addCategory metodu çalışıyor. Form
         * elementinden gelen Category nesne örneği,CategoryService aracılığıyla
         * Postgresql veritabanına kayıt ediliyor. Sonrasında ana sayfaya yönlendirme
         * yapıyoruz.
         */
    }

    @GetMapping("newCategory")
    public String newCategory(Model model) {
        model.addAttribute("category", new Category());
        return "newCategory";
    }

    @PostMapping("/addCategory")
    public String addCategory(Model model, @ModelAttribute("category") Category c) {
        // TODO Exception durumunu kontrol edip bir HTTP Status mesajı vermeyi
        // deneyebiliriz
        categoryService.add(c);
        return "redirect:/allCategories/";
    }
}