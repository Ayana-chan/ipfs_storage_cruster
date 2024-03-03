use axum::http;

#[inline]
pub fn move_entry_between_header_map(key: http::HeaderName,
                                     from_header_map: &http::HeaderMap,
                                     to_header_map: &mut http::HeaderMap,
) {
    if let Some(v) = from_header_map.get(&key) {
        to_header_map.insert(key, v.clone());
    }
}
