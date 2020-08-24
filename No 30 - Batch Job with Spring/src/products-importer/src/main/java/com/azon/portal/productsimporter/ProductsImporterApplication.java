package com.azon.portal.productsimporter;

import org.springframework.batch.core.Job;
import org.springframework.batch.core.JobParameters;
import org.springframework.batch.core.JobParametersBuilder;
import org.springframework.batch.core.launch.JobLauncher;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.scheduling.annotation.EnableScheduling;
import org.springframework.scheduling.annotation.Scheduled;

@SpringBootApplication
@EnableScheduling // Sonradan ekledik. İşin belli aralıklarla çalışmaya devam etmesini istiyorsak
					// eklememiz lazım
public class ProductsImporterApplication {

	@Autowired
	JobLauncher jobLauncher;

	@Autowired
	Job job;

	public static void main(String[] args) {
		SpringApplication.run(ProductsImporterApplication.class, args);
	}

	@Scheduled(cron = "0 */5 * * * ?") // Job 5 dakikada 1 çalışacak şekilde planlanmış durumda
	public void perform() throws Exception {
		JobParameters params = new JobParametersBuilder().addString("JobID", String.valueOf(System.currentTimeMillis()))
				.toJobParameters();
		jobLauncher.run(job, params);
	}
}
