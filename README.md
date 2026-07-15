# mr_kdtree_address

Reverse geocoding — finds nearest subdistrict (tambon/amphoe/province) from a lat/lng coordinate using a KD-tree.

> **Currently supports Thailand (TH) only.**

- 7,919 subdistrict points embedded at compile time (no runtime file I/O)
- Index builds once on first call via `OnceLock`
- O(log n) nearest-neighbor lookup

## Install

```toml
[dependencies]
mr_kdtree_address = "0.1"
```

## Usage

```rust
use mr_kdtree_address::{nearest, nearest_n};

// nearest subdistrict
let addr = nearest(13.7563, 100.5018).unwrap();
println!("{}", addr.addr_th);       // ต. ... อ. ... จ. กรุงเทพมหานคร
println!("{}", addr.addr_en);       // T. ..., A. ..., Changwat Bangkok
println!("{}", addr.province_th);   // จ. กรุงเทพมหานคร
println!("{}", addr.province_en);   // Bangkok
println!("{}", addr.amphoe_th);     // อ. ...
println!("{}", addr.tambon_th);     // ต. ...
println!("lat={} lng={}", addr.lat, addr.lng);

// nearest 3 subdistricts
let addrs = nearest_n(13.7563, 100.5018, 3);
for a in addrs {
    println!("{}", a.addr_en);
}
```

## Address fields

| Field         | Example                          |
|---------------|----------------------------------|
| `id`          | `5d4b1879a623db1ce05c663a`       |
| `addr_th`     | `ต. พระบรมมหาราชวัง อ. พระนคร จ. กรุงเทพมหานคร` |
| `addr_en`     | `T. Phra Borom Maha Ratchawang, A. Phra Nakhon, Changwat Bangkok` |
| `tambon_th`   | `ต. พระบรมมหาราชวัง`            |
| `tambon_en`   | `A. Phra Borom Maha Ratchawang` |
| `amphoe_th`   | `อ. พระนคร`                     |
| `amphoe_en`   | `T. Phra Nakhon`                 |
| `province_th` | `จ. กรุงเทพมหานคร`              |
| `province_en` | `Bangkok`                        |
| `lat`         | `13.7500`                        |
| `lng`         | `100.4913`                       |

## Notes

- Distance uses squared Euclidean on degrees (fast, accurate enough for subdistrict resolution)
- Data covers all 77 provinces of Thailand at tambon level
- Thread-safe — index is a global `OnceLock`

## License

MIT
