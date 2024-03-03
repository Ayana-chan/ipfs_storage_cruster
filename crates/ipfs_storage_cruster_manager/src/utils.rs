use axum::http;

pub fn move_entry_between_header_map(from_header_map: &http::HeaderMap,
                                   to_header_map: &mut http::HeaderMap,
                                   key: http::HeaderName) {
    if let Some(v) = from_header_map.get(&key) {
        to_header_map.insert(key, v.clone());
    }
}
