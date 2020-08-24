package com.azon.portal.productsimporter;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.batch.core.JobExecution;
import org.springframework.batch.core.BatchStatus;
import org.springframework.batch.core.listener.JobExecutionListenerSupport;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;


/*
    Burası Job içerisindeki işlerin icrası sırasında gerçekleşen olayları dinyelebileceğimiz nesne.
    Örneğin Job işini bitirdiğinde veya başladığında Elasticsearch'e bilgi logu atmak istersek uygun bir yer.
*/

@Component
public class JobListener extends JobExecutionListenerSupport {

    private static final Logger logger = LoggerFactory.getLogger(JobListener.class);

    @Autowired
    public JobListener() {
    }

    @Override
    public void beforeJob(JobExecution jobExecution) {
        logger.info("Aktarma işi çalışmaya başladı");
    }

    /*
     * Job çalışmasını bitirdiğinde devreye giren fonksiyonu override ettik. Statüyü
     * kontrol edip ekrana log basıyoruz.
     */
    @Override
    public void afterJob(JobExecution jobExecution) {
        if (jobExecution.getStatus() == BatchStatus.COMPLETED) {
            logger.info("Aktarma işlemi tamamlandı");
        }
    }
}