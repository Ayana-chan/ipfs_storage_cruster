Tiny crate to deserialize **form** style
from **query parameters** in http GET request.

# Able to Deserialize
1. **Form style simple array** (`/users?id=3,4,5`), whose elements' type T impls `FromStr` trait.
2. **Form style object** (`/users?id=role,admin,firstName,Alex`) that impls `FromStr` trait in a certain way (a little complex).

# Sample Code

## Deserialize Vec<T>
```rust
use deserialize_form_style_query_parameter::{form_vec_deserialize, option_form_vec_deserialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct QueryParams {
    id: Option<u32>,
    #[serde(deserialize_with = "form_vec_deserialize")]
    user_ids: Vec<u8>,
    #[serde(deserialize_with = "option_form_vec_deserialize", default)]
    user_names: Option<Vec<String>>,
}

fn main() {
    let correct_answer = QueryParams {
        id: Some(12345),
        user_ids: vec![1, 3],
        user_names: None,
    };
    // serde_urlencoded::from_str is executed by axum::extract::Query.
    // https://docs.rs/axum/latest/src/axum/extract/query.rs.html#87
    // So handler(Query(para)) also works.
    let example_params: QueryParams =
        serde_urlencoded::from_str("id=12345&user_ids=1,2b,3")
            .unwrap();
    assert_eq!(example_params, correct_answer);
}
```

## Deserialize Object
```rust
use deserialize_form_style_query_parameter::pure_from_str;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Address {
    city: String,
    postcode: String,
}

impl FromStr for Address {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // This function might be very complex in actual situations.
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 4{
            return Err(());
        }
        if !parts[0].eq("city") || !parts[2].eq("postcode") {
            return Err(());
        }
        Ok(Address{
            city: parts[1].to_string(),
            postcode: parts[3].to_string()
        })
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct QueryParams {
    id: Option<u32>,
    #[serde(deserialize_with = "pure_from_str")]
    address: Address,
}

fn main() {
    let correct_answer = QueryParams {
        id: Some(12345),
        address: Address {
            city: "Teyvat".to_string(),
            postcode: "191919".to_string()
        }
    };
    let example_params: QueryParams =
        serde_urlencoded::from_str("id=12345&address=city,Teyvat,postcode,191919")
            .unwrap();
    assert_eq!(example_params, correct_answer);
}
```

**For more details and usage , please refer to Document.**
