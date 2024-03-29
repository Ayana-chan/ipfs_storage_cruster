//! Tiny crate to deserialize **form** style data
//! from **query parameters** in http GET request.
//! Built upon **Serde**.
//!
//! # Able to Deserialize
//! 1. **Form style simple array** (`/users?id=3,4,5`), whose elements' type T impls `FromStr` trait.
//! 2. **Any type** (for example, **form object**: `/users?id=role,admin,firstName,Alex`) that impls `FromStr` trait in a certain way (a little complex).
//!
//! # Sample Code
//!
//! ## Deserialize Vec<T>
//! ```
//! use deserialize_form_style_query_parameter::{form_vec_deserialize, option_form_vec_deserialize};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, PartialEq, Deserialize, Serialize)]
//! struct QueryParams {
//!     id: Option<u32>,
//!     #[serde(deserialize_with = "form_vec_deserialize")]
//!     user_ids: Vec<u8>,
//!     #[serde(deserialize_with = "option_form_vec_deserialize", default)]
//!     user_names: Option<Vec<String>>,
//! }
//!
//! let correct_answer = QueryParams{
//!     id: Some(12345),
//!     user_ids: vec![1, 3],
//!     user_names: None,
//! };
//! // serde_urlencoded::from_str is executed by axum::extract::Query.
//! // https://docs.rs/axum/latest/src/axum/extract/query.rs.html#87
//! // So handler(Query(para)) also works.
//! let example_params: QueryParams =
//!     serde_urlencoded::from_str("id=12345&user_ids=1,2b,3")
//!     .unwrap();
//! assert_eq!(example_params, correct_answer);
//! ```
//!
//! ## Deserialize Object
//! ```
//! use deserialize_form_style_query_parameter::pure_from_str;
//! use serde::{Deserialize, Serialize};
//! use std::str::FromStr;
//!
//! #[derive(Debug, PartialEq, Deserialize, Serialize)]
//! struct Address {
//!     city: String,
//!     postcode: String,
//! }
//!
//! impl FromStr for Address {
//!     type Err = ();
//!
//!     fn from_str(s: &str) -> Result<Self, Self::Err> {
//!         // This function might be very complex in actual situations.
//!         let parts: Vec<&str> = s.split(',').collect();
//!         if parts.len() != 4{
//!             return Err(());
//!         }
//!         if !parts[0].eq("city") || !parts[2].eq("postcode") {
//!             return Err(());
//!         }
//!         Ok(Address{
//!             city: parts[1].to_string(),
//!             postcode: parts[3].to_string()
//!         })
//!     }
//! }
//!
//! #[derive(Debug, PartialEq, Deserialize, Serialize)]
//! struct QueryParams {
//!     id: Option<u32>,
//!     #[serde(deserialize_with = "pure_from_str")]
//!     address: Address,
//! }
//!
//! let correct_answer = QueryParams{
//!     id: Some(12345),
//!     address: Address {
//!         city: "Teyvat".to_string(),
//!         postcode: "191919".to_string()
//!     }
//! };
//! let example_params: QueryParams =
//!     serde_urlencoded::from_str("id=12345&address=city,Teyvat,postcode,191919")
//!     .unwrap();
//! assert_eq!(example_params, correct_answer);
//! ```
//!
//! # Provided Functions
//!
//! ## `fn form_vec_deserialize`
//! A deserialize function for `Vec<T: FromStr>`.
//!
//! Add `#[serde(deserialize_with = "form_vec_deserialize")]` above field to enable.
//!
//! This function use struct `FormVecVisitor` to deserialize,
//! which only accept string, and split it into a `Vec<String>` by ','.
//! Then execute `T::from_str(s).ok()` for every items (**illegal items will be discarded**).
//!
//! ## `fn option_form_vec_deserialize`
//! A deserialize function for `Option<Vec<T: FromStr>>`.
//!
//! Add `#[serde(deserialize_with = "form_vec_deserialize", default)]` above field to enable.
//! The `default` means this field would be `None` if its value is **not present**.
//!
//! More details about **serde's field attributes**: [serde.rs field-attrs](https://serde.rs/field-attrs.html).
//!
//! ## `fn pure_from_str`
//! A deserialize function for `T: FromStr`.
//!
//! Add `#[serde(deserialize_with = "pure_from_str")]` above field to enable.
//!
//! This function use struct `PureFromStrVisitor` to deserialize,
//! which only accept string, and just call `T::from_str`.
//! An error would be thrown when `T::from_str` failed.
//!
//! ## `fn option_pure_from_str`
//! A deserialize function for `Option<T: FromStr>`.
//!
//! Add `#[serde(deserialize_with = "option_pure_from_str", default)]` above field to enable.
//! The `default` means this field would be `None` if its value is **not present**.
//!
//! ## Adapt Custom Wrappers
//!
//! Take `option_form_vec_deserialize` source code as an example:
//! ```no_run
//! use std::str::FromStr;
//! use serde::Deserializer;
//! use deserialize_form_style_query_parameter::form_vec_deserialize;
//!
//! pub fn option_form_vec_deserialize<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
//!     where D: Deserializer<'de>,
//!           T: FromStr {
//!     form_vec_deserialize(deserializer).map(Some)
//! }
//! ```
//! Copy and modify it to meet your needs.
//!
//! # When and Why Use This Crate
//!
//! ## When and Why use form style query
//! When an API must require form style ([swagger spec here](https://swagger.io/docs/specification/serialization/))，
//! we don't have any other choice.
//!
//! ## A serious bug
//! Http GET request in form style are something like `http://www.just.example/path?address=99&name=55,66`,
//! which represents data structure like:
//! ```json
//! {
//!     "address" = 99,
//!     "name" = [55, 66]
//! }
//! ```
//! For backend, we have to create a handler to handle this GET request, for example:
//! ```ignore
//! use axum::extract::Query;
//!
//! async fn example_api(
//!         token: Token,
//!         Query(my_query_args): Query<MyQueryArgs>
//!     ) -> ApiResponse {
//!     println!("get args: {:?}", my_query_args);
//!     // do something
//!     ApiResponse::new()
//! }
//! ```
//! But you will receive a confusing error message as response:
//! ```txt
//! invalid type: string "55,66", expected a sequence
//! ```
//!
//! ## Why bug
//! In [axum::extract::Query source code](https://docs.rs/axum/latest/src/axum/extract/query.rs.html),
//! we found that the `from_request_parts` use `serde_urlencoded::from_str` to deserialize query parameters.
//! This crate is not powerful enough to deserialize complex query parameters.
//!
//! After replacing `serde_urlencoded` with `serde_qs`, which claiming to be skilled in handling query parameters,
//! nothing changed. Crate `serde_qs` only support array like`arr[]=55&arr[]=66` and `arr[0]=55&arr[1]=66`.
//! This might be an [unresolved bug](https://github.com/samscott89/serde_qs/issues/83).
//!
//! Very similarly, both of them can't recognize form style object.
//!
//! ## Why this crate works
//! Luckily, both `serde_urlencoded::from_str` and `serde_qs::from_str`
//! recognize form style arrays and objects as Strings, and then execute their deserialize functions.
//!
//! So this crate provide some deserialize functions, and user could use `deserialize_with`
//! to enable them.
//!

use std::marker::PhantomData;
use std::str::FromStr;
use serde::Deserializer;
use serde::de::{Error, Visitor};

pub fn option_form_vec_deserialize<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
    where D: Deserializer<'de>,
          T: FromStr {
    form_vec_deserialize(deserializer).map(Some)
}

pub fn form_vec_deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where D: Deserializer<'de>,
          T: FromStr {
    deserializer.deserialize_str(FormVecVisitor::<T>::new())
}

pub struct FormVecVisitor<T> {
    marker: PhantomData<fn() -> Vec<T>>,
}

impl<T> FormVecVisitor<T> {
    fn new() -> Self {
        Self {
            marker: PhantomData
        }
    }
}

impl<'de, T> Visitor<'de> for FormVecVisitor<T>
    where
        T: FromStr
{
    // The type that our Visitor is going to produce.
    type Value = Vec<T>;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("form style array in GET http request's query parameter")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        if v.is_empty() {
            return Ok(Vec::new());
        }
        let arr = v.split(',')
            .filter_map(|s| {
                // value: Custom("invalid type: string \"1\", expected u8")
                // String is ok, but u8 is not. It seems that this method has not actually undergone type conversion
                // let strdes: StrDeserializer<E> = s.into_deserializer();
                // let ans = T::deserialize::<_>(strdes).unwrap();
                // ans
                T::from_str(s).ok()
            }).collect();
        Ok(arr)
    }
}

pub fn pure_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where D: Deserializer<'de>,
          T: FromStr {
    deserializer.deserialize_str(PureFromStrVisitor::<T>::new())
}

pub fn option_pure_from_str<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where D: Deserializer<'de>,
          T: FromStr {
    pure_from_str(deserializer).map(Some)
}

pub struct PureFromStrVisitor<T> {
    marker: PhantomData<fn() -> T>,
}

impl<T> PureFromStrVisitor<T> {
    fn new() -> Self {
        Self {
            marker: PhantomData
        }
    }
}

impl<'de, T> Visitor<'de> for PureFromStrVisitor<T>
    where
        T: FromStr
{
    // The type that our Visitor is going to produce.
    type Value = T;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("form style object in GET http request's query parameter")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        T::from_str(v).map_err(|_| E::custom(format!("Failed deserialize pure str: {}", v)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct Address {
        city: String,
        postcode: String,
    }

    impl FromStr for Address {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // fake logic
            Ok(Address {
                city: s.to_string(),
                postcode: "yyy".to_string(),
            })
            // Err(())
        }
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct QueryParams {
        id: Option<u32>,
        #[serde(deserialize_with = "pure_from_str")]
        address: Address,
        #[serde(deserialize_with = "form_vec_deserialize")]
        user_ids: Vec<u8>,
        #[serde(deserialize_with = "option_form_vec_deserialize", default)]
        user_names: Option<Vec<String>>,
    }

    #[test]
    fn it_works() {
        let rec_params: QueryParams =
            serde_urlencoded::from_str("id=12345&address=city,abc,postcode,efg&user_ids=1a,2,3")
                .unwrap();
        println!("{:#?}", rec_params);
    }
}
