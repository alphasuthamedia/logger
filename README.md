https://github.com/alphasuthamedia/logger

jadi gini, dulu runner (bg / daemon) buat server saya itu pakai C sama shell/Bash. sederhana aja, cuma capture stdout ke logfile sama stderrnya. setelah ambil matkul AAW kemarin dapat kafka, ngobbrol sama om syahrul, plus makin lama makin banyak yang perlu dimanage, butuh orkestrasi yang enak, cobain rust via rdkafka eh enak... execute sheell jg gampang pakai Command::new().args() dkk, terus mayoritas jg udah  bisa di rust, gak perlu cross compile smaa codebase C lama.... meskpun demikian, rdfkafka cuma wrapper librdkafka (kalau gak salah namanya ini) yang ttp ditulis pakai C, yaudah tinggal ganti linker sama header (kalau perlu / compile beda di arch) aja...

 

Implementasikan sistem event-driven sederhana menggunakan Kafka atau RabbitMQ untuk mengirim dan memproses event antar komponen aplikasi - PASSED (pake kafka, knapa gak MQ krn kemarin PPL aku bikin buat kirim logging, MQ kan cuma queue, kalau Kafka kan ref.. bisa dibikin Queue jg, tapi kompabilitasnya lebih banyak, aku jg udah pernah pakai RabbitMQ di TRUI, jadi gak seru aja, soal batching, polling / flush, async, dkk Kafka tdk bisa diragukan, group, partition, mesh setup, replication dll mantab dah)

 

Buat minimal satu producer dan satu consumer yang saling berkomunikasi melalui message broke - PASSED (1 consumer, up ke telegram group pribadi saya, producernya sementara cm itu aja, masih refaktor pelan2..., soalna jg cuma hobbi aja)

 

Demonstrasikan bagaimana event dikirim, disimpan dalam topic/queue, dan diproses oleh consumer secara real-time - PASSED (event datang, lalau producer ngesend ke buffer, waktu dipolling makan topic lokal ditrigger untuk kirim ke broker, terus diambil oleh consumer) lebih kompleks (siap app itu punya thread sendiri yang unblocking via Arc, (balik lagi ke mekanisme yang atas), consumer ambil,  kalau berhasil dikiim ke telegram, maka dicommit (dianggap ini sudah dibaca / pointer dimajukan, ini juga menjadi alasan kenapa pakai Kafka)

 

Lakukan pengujian dengan mengirim beberapa event dan jelaskan bagaimana mekanisme komunikasi asynchronous tersebut bekerja serta apa perbedaannya dengan komunikasi request–response biasa (akan dikerjakan)
