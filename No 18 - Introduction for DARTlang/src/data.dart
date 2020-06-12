/*
basit bir library örneği. 
Library'yi namespace veya paket gibi düşünmek mümkün

burada çok basit bir Abstract Factory tasarım kalıbı örneği uygulanmaktadır.
Dart'ın birkaç nesne yönelimli davranışını anlamakttır amaç.

senaryo: Departmana göre rapor isteyen bir object user varmış. İlgilendiği raporların üretilme detayları onun için önemli değil.
Bu üretim işini fabrika nesneleri üstlenecek. Budget, Performance ve Employees örnek raporlar. Bunları üretme işi ise ReportFactory 
türevli fabrika nesnelerinde.
*/

library data;

// abstrat class
abstract class Report {
  void publish();
}

// concrete class
class Budget implements Report {
  void publish() {
    print("Bütçe raporu dağıtılıyor");
  }
}

// concrete class
class Performance implements Report {
  void publish() {
    print("DB Performans raporu dağıtılıyor");
  }
}

// concrete class
class Employees implements Report {
  void publish() {
    print("Çalışan performans raporu dağıtılıyor");
  }
}

// factory
// rapor üretme işini tanımlayan sözleşme
abstract class ReportFactory {
  Report
      CreateReport(); //Report döndürüyor. Yani Budget, Performance ve Employees için kullanılabilir
}

//concrete factory sınıfları
// Bütçe raporu üretme işini üstlenen fabrika
class BudgetReportFactory implements ReportFactory {
  Budget CreateReport() {
    return Budget();
  }
}

class PerformanceReportFactory implements ReportFactory {
  Performance CreateReport() {
    return Performance();
  }
}

class EmployeesReportFactory implements ReportFactory {
  Employees CreateReport() {
    return Employees();
  }
}
