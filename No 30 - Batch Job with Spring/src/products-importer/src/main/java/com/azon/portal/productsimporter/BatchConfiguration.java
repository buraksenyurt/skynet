package com.azon.portal.productsimporter;

import org.springframework.batch.core.Job;
import org.springframework.batch.core.Step;
import org.springframework.batch.core.configuration.annotation.EnableBatchProcessing;
import org.springframework.batch.core.configuration.annotation.JobBuilderFactory;
import org.springframework.batch.core.configuration.annotation.StepBuilderFactory;
import org.springframework.batch.core.launch.support.RunIdIncrementer;
import org.springframework.batch.item.file.FlatFileItemReader;
import org.springframework.batch.item.file.MultiResourceItemReader;
import org.springframework.batch.item.file.builder.FlatFileItemReaderBuilder;
import org.springframework.batch.item.file.builder.MultiResourceItemReaderBuilder;
import org.springframework.batch.item.file.mapping.BeanWrapperFieldSetMapper;
import org.springframework.batch.item.json.JacksonJsonObjectMarshaller;
import org.springframework.batch.item.json.JsonFileItemWriter;
import org.springframework.batch.item.json.builder.JsonFileItemWriterBuilder;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.io.FileSystemResource;
import org.springframework.core.io.Resource;

/*
    Batch akışının tasarlandığı sınıf burası. Genel olarak bir Job ve bu Job'un çalıştırdığı adımlar oluyor.
    Bu nedenle JobBuilderFactory ve StepBuilderFactory nesneleri başta belirtilmiş durumda.
*/

@Configuration
@EnableBatchProcessing
public class BatchConfiguration {

    @Autowired
    public JobBuilderFactory jobBuilderFactory;

    @Autowired
    public StepBuilderFactory stepBuilderFactory;

    // Resources altındaki tüm csv dosyalarını tutacağımız dizi
    @Value("*.csv")
    private Resource[] inputFiles;

    // Çıktının yazılacağı dosyayı da bir Resource olarak kullanacağız
    private Resource outputResource = new FileSystemResource("output.json");

    /*
     * Senaryodaki amacımız hatırlanacağı üzere n sayıda CSV dosyasını alıp, bunları
     * tek bir JSON dosyasına dönüştürmekti.
     * 
     * Bu iş parçası inputFiles kaynağında yer alan ne kadar CSV varsa onları okuyup
     * Product nesnelerin dönüştürmek üzere bir alttaki iş parçasına(reader)
     * devrediyor (setDelegate fonksiyonuna dikkat)
     */
    @Bean
    public MultiResourceItemReader<Product> multiResourceItemReader() {
        return new MultiResourceItemReaderBuilder<Product>().name("multiResourceItemReader").resources(inputFiles)
                .delegate(reader()).build();
    }

    /*
     * Bu iş parçası resource olarak gelen içeriği parse edip Product nesne
     * örnekleri haline getirmekte. Üstteki iş parçası tarafından tetikleniyor.
     */
    @Bean
    public FlatFileItemReader<Product> reader() {
        return new FlatFileItemReaderBuilder<Product>().name("reader") // İş parçasına bir isim verdik
                .delimited().names(new String[] { "name", "unitPrice", "quantity" }) // sütun adları
                .fieldSetMapper(new BeanWrapperFieldSetMapper<Product>() {
                    {
                        // Hedef tip Product sınıfı
                        setTargetType(Product.class);
                    }
                }).build();
    }

    /*
     * CSV'ten okunan satırların Product nesnesi olarak elde edilişi sırasında belki araya girmek isteyebiliriz.
     * Araya girip veriyi değiştirip yeni Product nesnesi olarak yola devam etmesini isteyebiliriz.
     */
    @Bean
    public ProductItemConvertor processor() {
        return new ProductItemConvertor();
    }

    /*
     * Bu iş parçası ise artık Product nesnesine dönüşmüş CSV içeriklerinin JSON formatına serileştirilip
     * dosya olarak yazılmasını sağlıyor.
     */
    @Bean
    public JsonFileItemWriter<Product> writer() {
        return new JsonFileItemWriterBuilder<Product>().jsonObjectMarshaller(new JacksonJsonObjectMarshaller<>())
                .resource(outputResource).name("writer").build();
    }

    /*
     * Aşağıdaki metotlar da sırasıyla Job ve içerisine dahil olacak adımları icra etmekte
     */

    @Bean
    public Job csvToJsonJob(JobListener listener, Step firstStep) {
        return jobBuilderFactory.get("csvToJsonJob").incrementer(new RunIdIncrementer()).listener(listener)
                .flow(firstStep).end().build();
    }

    @Bean
    public Step firstStep() {
        //chunk ile bir anda kaç satır veriyi işleyeceğimizi belirttik.
        return stepBuilderFactory.get("firstStep").listener(new JobListener())
                .<Product, Product>chunk(50).reader(multiResourceItemReader()).processor(processor()).writer(writer())
                .build();
    }
}