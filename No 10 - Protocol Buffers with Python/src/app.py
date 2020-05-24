# encoding:utf-8

import jobs_pb2 as Job  # Protoc dosyasından üretilen paketi ekledik

# Yeni bir Job nesnesi tanımlayıp içine birkaç Task ekliyoruz
job = Job.Job()
job.job_id = 1802234
job.title = "Şube personeli fatura onay sürecine olarak belge yükleyebileceğim bir ekran istiyorum."

task1 = job.tasks.add()
task1.task_id = 1
task1.title = "Ekran Tasarımı"
task1.description = "Fatura yükleme ekranının tasarımı"
task1.state = Job.State.Value("IN_PROGRESS")

task2 = job.tasks.add()
task2.task_id = 2
task2.title = "Fatura Şema Tasarımı"
task2.description = "Fatura içeriği için gerekli protoc içeriğinin tasarımı"
task2.state = Job.State.Value("PLANNED")

# print(job)  # Ekrana çıktıyı basalım

# Job içeriğini jobs.data isimli dosya içine serileştiriyoruz
with open("./jobs.data", "wb") as file:
    file.write(job.SerializeToString())

print("İçerik Serileştirildi...")
print(job.SerializeToString())
print()

incoming_job = Job.Job()  # yeni bir job nesnesi oluşturalım
# jobs.data içeriğini okuyup incoming_job içerisine ters serileştirelim
with open("./jobs.data", "rb") as file:
    incoming_job.ParseFromString(file.read())

print("Şimdi Dosyadan Ters Serileştirildi")
print(incoming_job)
