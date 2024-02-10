pub struct HttpHeaderPorterFromReqwest<'a> {
    header: Option<axum::http::HeaderMap>,
    reqwest_header_map: &'a reqwest::header::HeaderMap,
}

impl<'a> HttpHeaderPorterFromReqwest<'a> {
    pub fn new(reqwest_header_map: &'a reqwest::header::HeaderMap) -> Self {
        HttpHeaderPorterFromReqwest {
            header: Some(axum::http::HeaderMap::new()),
            reqwest_header_map,
        }
    }

    pub fn transfer_when_exist_with_static_key(mut self, key: &'static str) -> Self {
        let header_value = self.reqwest_header_map
            .get(key);
        if let Some(header_value) = header_value {
            let hv = axum::http::HeaderValue::from_bytes(header_value.as_ref());
            if let Ok(hv) = hv {
                self.header.as_mut().unwrap().insert(key, hv);
            }
        }
        //     .map(|v|
        //         v.to_str().unwrap_or_default()
        //     );
        // if let Some(v) = header_value {
        //     let hv = axum::http::HeaderValue::from_str(v);
        //     if let Ok(hv) = hv {
        //         self.header.as_mut().unwrap().insert(key, hv);
        //     }
        // }

        self
    }

    pub fn finish(self) -> axum::http::HeaderMap {
        self.header.unwrap()
    }
}

