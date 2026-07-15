use csv::ReaderBuilder;
use kdtree::distance::squared_euclidean;
use kdtree::KdTree;
use std::sync::OnceLock;

const CSV_DATA: &str = include_str!("../data/geo_locations.csv");

#[derive(Debug, Clone)]
pub struct Address {
    pub id: String,
    pub addr_th: String,
    pub addr_en: String,
    pub amphoe_th: String,
    pub amphoe_en: String,
    pub province_th: String,
    pub province_en: String,
    pub tambon_th: String,
    pub tambon_en: String,
    pub lat: f64,
    pub lng: f64,
}

struct GeoIndex {
    tree: KdTree<f64, usize, [f64; 2]>,
    addrs: Vec<Address>,
}

static INDEX: OnceLock<GeoIndex> = OnceLock::new();

fn index() -> &'static GeoIndex {
    INDEX.get_or_init(|| {
        let mut addrs: Vec<Address> = Vec::new();
        let mut tree: KdTree<f64, usize, [f64; 2]> = KdTree::new(2);

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(CSV_DATA.as_bytes());

        for record in rdr.records().flatten() {
            let lat: f64 = record.get(5).and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let lng: f64 = record.get(6).and_then(|v| v.parse().ok()).unwrap_or(0.0);
            if lat == 0.0 && lng == 0.0 {
                continue;
            }
            let idx = addrs.len();
            addrs.push(Address {
                id: record.get(0).unwrap_or_default().to_string(),
                addr_th: record.get(1).unwrap_or_default().to_string(),
                addr_en: record.get(2).unwrap_or_default().to_string(),
                amphoe_th: record.get(3).unwrap_or_default().to_string(),
                amphoe_en: record.get(4).unwrap_or_default().to_string(),
                lat,
                lng,
                province_th: record.get(7).unwrap_or_default().to_string(),
                province_en: record.get(8).unwrap_or_default().to_string(),
                tambon_th: record.get(9).unwrap_or_default().to_string(),
                tambon_en: record.get(10).unwrap_or_default().to_string(),
            });
            let _ = tree.add([lat, lng], idx);
        }

        GeoIndex { tree, addrs }
    })
}

/// Returns nearest subdistrict address for given lat/lng.
pub fn nearest(lat: f64, lng: f64) -> Option<&'static Address> {
    let idx = &index();
    idx.tree
        .nearest(&[lat, lng], 1, &squared_euclidean)
        .ok()
        .and_then(|r| r.into_iter().next())
        .map(|(_, i)| &idx.addrs[*i])
}

/// Returns nearest n subdistrict addresses for given lat/lng.
pub fn nearest_n(lat: f64, lng: f64, n: usize) -> Vec<&'static Address> {
    let idx = &index();
    idx.tree
        .nearest(&[lat, lng], n, &squared_euclidean)
        .unwrap_or_default()
        .into_iter()
        .map(|(_, i)| &idx.addrs[*i])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bangkok_center() {
        let addr = nearest(13.7563, 100.5018).expect("must find address");
        assert!(
            addr.province_en.contains("Bangkok") || addr.province_th.contains("กรุงเทพ"),
            "expected Bangkok, got: {}",
            addr.province_en
        );
    }

    #[test]
    fn nearest_n_returns_n() {
        let results = nearest_n(13.7563, 100.5018, 3);
        assert_eq!(results.len(), 3);
    }
}
