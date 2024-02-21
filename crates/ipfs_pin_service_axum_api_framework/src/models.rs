use axum::response::{IntoResponse, Response};

/// Pin object
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
pub struct Pin {
    /// Content Identifier (CID) to be pinned recursively
    #[serde(rename = "cid")]
    pub cid: String,

    /// Optional name for pinned data; can be used for lookups later
    #[serde(rename = "name")]
    #[validate(
    length(max = 255),
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A list of known sources (providers) of the data.
    /// Sent by a client in a pin request.
    /// Pinning service will try to connect to them to speed up data transfer. \
    /// Addresses provided in origins list are relevant only during the initial pinning,
    /// and don't need to be persisted by the pinning service.
    #[serde(rename = "origins")]
    #[validate(
    length(min = 0, max = 20),
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<String>>,

    /// Optional metadata for pin object
    #[serde(rename = "meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::HashMap<String, String>>,
}

impl Pin {
    #[allow(clippy::new_without_default)]
    pub fn new(cid: String, name: Option<String>, origins: Option<Vec<String>>, meta: Option<std::collections::HashMap<String, String>>) -> Pin {
        Pin {
            cid,
            name,
            origins,
            meta,
        }
    }
}

/// Converts the Pin value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Pin {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("cid".to_string()),
            Some(self.cid.to_string()),
            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),
            self.origins.as_ref().map(|origins| {
                [
                    "origins".to_string(),
                    origins.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping meta in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Pin value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Pin {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub cid: Vec<String>,
            pub name: Vec<String>,
            pub origins: Vec<Vec<String>>,
            pub meta: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Pin".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "cid" => intermediate_rep.cid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "origins" => return std::result::Result::Err("Parsing a container in this style is not supported in Pin".to_string()),
                    "meta" => return std::result::Result::Err("Parsing a container in this style is not supported in Pin".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Pin".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Pin {
            cid: intermediate_rep.cid.into_iter().next().ok_or_else(|| "cid missing in Pin".to_string())?,
            name: intermediate_rep.name.into_iter().next(),
            origins: intermediate_rep.origins.into_iter().next(),
            meta: intermediate_rep.meta.into_iter().next(),
        })
    }
}

/// Response used for listing pin objects matching request
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
pub struct PinResults {
    /// The total number of pin objects that exist for passed query filters
    #[serde(rename = "count")]
    #[validate(
    range(min = 0),
    )]
    pub count: u32,

    /// An array of PinStatus results
    #[serde(rename = "results")]
    #[validate(
    length(min = 0, max = 1000),
    )]
    pub results: Vec<PinStatus>,
}


impl PinResults {
    #[allow(clippy::new_without_default)]
    pub fn new(count: u32, results: Vec<PinStatus>) -> PinResults {
        PinResults {
            count,
            results,
        }
    }
}

impl IntoResponse for PinResults {
    fn into_response(self) -> Response {
        todo!()
    }
}

/// Converts the PinResults value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PinResults {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("count".to_string()),
            Some(self.count.to_string()),

            // Skipping results in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PinResults value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PinResults {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub count: Vec<u32>,
            pub results: Vec<Vec<PinStatus>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PinResults".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "count" => intermediate_rep.count.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "results" => return std::result::Result::Err("Parsing a container in this style is not supported in PinResults".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing PinResults".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PinResults {
            count: intermediate_rep.count.into_iter().next().ok_or_else(|| "count missing in PinResults".to_string())?,
            results: intermediate_rep.results.into_iter().next().ok_or_else(|| "results missing in PinResults".to_string())?,
        })
    }
}

/// Pin object with status
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
pub struct PinStatus {
    /// Globally unique identifier of the pin request; can be used to check the status of ongoing pinning, or pin removal
    #[serde(rename = "requestid")]
    pub requestid: String,

    #[serde(rename = "status")]
    pub status: Status,

    /// Immutable timestamp indicating when a pin request entered a pinning service; can be used for filtering results and pagination
    #[serde(rename = "created")]
    pub created: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "pin")]
    pub pin: Pin,

    /// A list of temporary destination (retrievers) for the data.
    /// Returned by pinning service in a response for a pin request.
    /// These peers are provided by a pinning service for the purpose of fetching data about to be pinned.
    #[serde(rename = "delegates")]
    #[validate(
    length(min = 1, max = 20),
    )]
    pub delegates: Vec<String>,

    /// Optional info for PinStatus response
    #[serde(rename = "info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<std::collections::HashMap<String, String>>,

}


impl PinStatus {
    #[allow(clippy::new_without_default)]
    pub fn new(requestid: String, status: Status, created: chrono::DateTime::<chrono::Utc>, pin: Pin, delegates: Vec<String>, info: Option<std::collections::HashMap<String, String>>) -> PinStatus {
        PinStatus {
            requestid,
            status,
            created,
            pin,
            delegates,
            info,
        }
    }
}

/// Converts the PinStatus value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PinStatus {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            Some("requestid".to_string()),
            Some(self.requestid.to_string()),

            // Skipping status in query parameter serialization

            // Skipping created in query parameter serialization

            // Skipping pin in query parameter serialization


            Some("delegates".to_string()),
            Some(self.delegates.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

            // Skipping info in query parameter serialization
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PinStatus value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PinStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub requestid: Vec<String>,
            pub status: Vec<Status>,
            pub created: Vec<chrono::DateTime::<chrono::Utc>>,
            pub pin: Vec<Pin>,
            pub delegates: Vec<Vec<String>>,
            pub info: Vec<std::collections::HashMap<String, String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PinStatus".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "requestid" => intermediate_rep.requestid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<Status as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "created" => intermediate_rep.created.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pin" => intermediate_rep.pin.push(<Pin as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "delegates" => return std::result::Result::Err("Parsing a container in this style is not supported in PinStatus".to_string()),
                    "info" => return std::result::Result::Err("Parsing a container in this style is not supported in PinStatus".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing PinStatus".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PinStatus {
            requestid: intermediate_rep.requestid.into_iter().next().ok_or_else(|| "requestid missing in PinStatus".to_string())?,
            status: intermediate_rep.status.into_iter().next().ok_or_else(|| "status missing in PinStatus".to_string())?,
            created: intermediate_rep.created.into_iter().next().ok_or_else(|| "created missing in PinStatus".to_string())?,
            pin: intermediate_rep.pin.into_iter().next().ok_or_else(|| "pin missing in PinStatus".to_string())?,
            delegates: intermediate_rep.delegates.into_iter().next().ok_or_else(|| "delegates missing in PinStatus".to_string())?,
            info: intermediate_rep.info.into_iter().next(),
        })
    }
}

/// Status a pin object can have at a pinning service. \
/// - `queued` is passive: the pin was added to the queue but the service isn't consuming any resources to retrieve it yet. \
/// - `pinning` is active: the pinning service is trying to retrieve the CIDs by finding providers for all involved CIDs, connect to these providers and download data from them. \
///
/// When a new pin object is created it typically starts in a `queued` state.
/// Once the pinning service actively seeks to retrieve the file it changes to `pinning`.
/// `pinning` typically means that the data behind `Pin.cid` was not found on the pinning service and is being fetched from the IPFS network at large, which may take time.
/// In either case, the user can periodically check pinning progress via `GET /pins/{requestid}` until pinning is successful,
/// or the user decides to remove the pending pin. \
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum Status {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "pinning")]
    Pinning,
    #[serde(rename = "pinned")]
    Pinned,
    #[serde(rename = "failed")]
    Failed,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Status::Queued => write!(f, "queued"),
            Status::Pinning => write!(f, "pinning"),
            Status::Pinned => write!(f, "pinned"),
            Status::Failed => write!(f, "failed"),
        }
    }
}

impl std::str::FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "queued" => std::result::Result::Ok(Status::Queued),
            "pinning" => std::result::Result::Ok(Status::Pinning),
            "pinned" => std::result::Result::Ok(Status::Pinned),
            "failed" => std::result::Result::Ok(Status::Failed),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Alternative text matching strategy
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum TextMatchingStrategy {
    #[serde(rename = "exact")]
    Exact,
    #[serde(rename = "iexact")]
    Iexact,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "ipartial")]
    Ipartial,
}

impl std::fmt::Display for TextMatchingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TextMatchingStrategy::Exact => write!(f, "exact"),
            TextMatchingStrategy::Iexact => write!(f, "iexact"),
            TextMatchingStrategy::Partial => write!(f, "partial"),
            TextMatchingStrategy::Ipartial => write!(f, "ipartial"),
        }
    }
}

impl std::str::FromStr for TextMatchingStrategy {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "exact" => std::result::Result::Ok(TextMatchingStrategy::Exact),
            "iexact" => std::result::Result::Ok(TextMatchingStrategy::Iexact),
            "partial" => std::result::Result::Ok(TextMatchingStrategy::Partial),
            "ipartial" => std::result::Result::Ok(TextMatchingStrategy::Ipartial),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}
