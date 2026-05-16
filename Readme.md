### Experiment 1.2: Understanding how it works

![Experiment 1.2](assets/experiment-1-2.png)

**Penjelasan mengapa output "hey hey" muncul terlebih dahulu:**
Hal ini terjadi karena pada baris kode `spawner.spawn(...)`, program sebenarnya hanya mendaftarkan/memasukkan fungsi `async` (yang berisi `howdy!` dan `done!`) ke dalam antrean (*queue*) untuk dieksekusi nanti.

Karena sifatnya *asynchronous*, *thread* utama (*main thread*) tidak menunggu antrean itu selesai, melainkan langsung lanjut mengeksekusi baris kode sinkronus yang ada di bawahnya, yaitu mencetak `"Tsaniya's Computer: hey hey"`.

Setelah itu, barulah program memanggil `executor.run()`. Fungsi inilah yang bertugas menjalankan tugas-tugas yang tadi sudah masuk ke dalam antrean. Oleh karena itu, `"howdy!"` dan `"done!"` baru dieksekusi dan dicetak belakangan.

---
### Experiment 1.3: Multiple Spawn and removing drop

![Experiment 1.3 Without Drop](assets/experiment-1-3-without-drop.png)
![Experiment 1.3 With Drop](assets/experiment-1-3-with-drop.png)

**Penjelasan Multiple Spawn:**
Saat menambahkan beberapa `spawner.spawn`, program mencetak semua pesan "howdy" secara berurutan, melakukan *delay* 2 detik secara bersamaan, lalu mencetak semua pesan "done" . Hal ini menunjukkan bahwa tugas-tugas *async* tersebut dijalankan secara konkuren (bersamaan) oleh satu *thread* utama.

**Penjelasan mengapa fungsi `drop` penting:**
Fungsi `drop(spawner)` digunakan untuk menutup saluran (*channel*) komunikasi antara *spawner* (pengirim tugas) dan *executor* (penerima/pengeksekusi tugas).
Ketika baris `drop(spawner)` dihapus atau di- *comment*, *executor* tidak pernah menerima sinyal bahwa pengiriman tugas sudah selesai. Akibatnya, fungsi `executor.run()` akan terus berjalan tanpa henti (terblokir/nge-*hang*) karena selalu menunggu tugas baru yang tidak akan pernah datang dari *channel* tersebut .
