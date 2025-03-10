# Distributed Inventory System

Sistem manajemen stok real-time yang terdistribusi untuk mall, manufaktur, atau hotel. Menggunakan algoritma konsensus (Raft/Paxos) untuk sinkronisasi data antar node.

## Manfaat
- Mengurangi overstock/understock
- Cocok untuk bisnis besar

## Struktur Folder
```
inventory-system/
├── src/
│   ├── rust-core/          # Backend performa tinggi (Rust)
│   │   ├── consensus.rs    # Implementasi Raft
│   │   ├── network.rs      # Komunikasi antar node
│   │   └── storage.rs      # Penyimpanan lokal
│   ├── haskell-logic/      # Logika bisnis (Haskell)
│   │   ├── Stock.hs        # Model stok
│   │   └── Sync.hs         # Sinkronisasi data
├── tests/                  # Unit dan integration tests
├── docs/                   # Dokumentasi API dan arsitektur
└── README.md
```

## Cara Menjalankan
1. Clone repository ini:
   ```sh
   git clone https://github.com/svenhven/distributed-inventory-system.git
   cd distributed-inventory-system
   ```

2. Install Rust dan Haskell:
   - **Rust:**
     ```sh
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     source $HOME/.cargo/env
     ```
   - **Haskell:**
     ```sh
     sudo apt update
     sudo apt install -y ghc cabal-install
     cabal update
     ```

3. Jalankan backend Rust:
   ```sh
   cd src/rust-core
   cargo run
   ```

4. Kompilasi dan jalankan logika bisnis Haskell:
   ```sh
   cd src/haskell-logic
   ghc -o Sync Sync.hs
   ./Sync
   ```

## Kontribusi
1. Fork repository ini.
2. Buat branch fitur baru (`git checkout -b feature-branch`).
3. Commit perubahan Anda (`git commit -m 'Add some feature'`).
4. Push ke branch (`git push origin feature-branch`).
5. Buat pull request baru.

## Lisensi
Proyek ini dilisensikan di bawah [MIT License](./LICENSE).