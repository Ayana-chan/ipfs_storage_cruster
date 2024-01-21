//! Tiny crate to deserialize array (in **form** style) from **query parameters** in http GET request.
//!
//! # NOTE
//! Deserialize result is `Vec<T: FromStr>`.
//! The deserialization of `T` is based on `FromStr`, so make sure you have done `impl FromStr for T` correctly.
//!
//! # Sample Code
//!
//! ```
//! use deserialize_form_array_in_query_parameter::{form_vec_deserialize, option_form_vec_deserialize};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, PartialEq, Deserialize, Serialize)]
//! struct Address {
//!     city: String,
//!     postcode: String,
//! }
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
//! # Provided Items
//!
//! ## `fn form_vec_deserialize`
//! A deserialize function for `Vec<T: FromStr>`, which could be used by `deserialize_with`. \
//! Add `#[serde(deserialize_with = "form_vec_deserialize")]` above field to enable.
//!
//! ## `fn option_form_vec_deserialize`
//! A deserialize function for `Option<Vec<T: FromStr>>`, which could be used by `deserialize_with`. \
//! Add `#[serde(deserialize_with = "form_vec_deserialize", default)]` above field to enable.
//! The `default` means this field would be `None` if its value is **not present**. \
//! More details about **serde's field attributes**: [serde.rs field-attrs](https://serde.rs/field-attrs.html).
//!
//! ## `struct FormVecVisitor`
//! `FormVecVisitor<T>` is the visitor for `Vec<T>` (only available when `T: FromStr`).
//! `form_vec_deserialize` only call `deserializer.deserialize_str(FormVecVisitor::<T>::new())`
//! to deserialize `Vec<T>`.\
//! `FormVecVisitor` only accept string, and just split it into a String Vec by ','.
//! Then execute `T::from_str(s).ok()` for every items (illegal items will be discarded).
//!
//! ## Adapt Custom Vec Wrappers
//!
//! Taking `option_form_vec_deserialize`'s source code as an example:
//! ```no_run
//! use std::str::FromStr;
//! use serde::Deserializer;
//! use deserialize_form_array_in_query_parameter::form_vec_deserialize;
//!
//! pub fn option_form_vec_deserialize<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
//!     where D: Deserializer<'de>,
//!           T: FromStr {
//!     form_vec_deserialize(deserializer).map(|v| Some(v))
//! }
//! ```
//! Copy it and modify `map(|v| Some(v))` and `Result<Option<Vec<T>>, D::Error>` to meet your needs.
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
//! This crate is not powerful enough to deserialize complex query parameters. \
//! After replacing `serde_urlencoded` with `serde_qs`, which claiming to be skilled in handling query parameters,
//! nothing changed. Crate `serde_qs` only support array like`arr[]=55&arr[]=66` and `arr[0]=55&arr[1]=66`.
//! This might be an [unresolved bug](https://github.com/samscott89/serde_qs/issues/83). \
//!
//! ## Why this crate works
//! Luckily, both `serde_urlencoded::from_str` and `serde_qs::from_str`
//! recognize form arrays as Strings, and then execute their deserialize functions. \
//! So this crate provide customize deserialize functions, and we can use `deserialize_with`
//! to enable them.
//!

use std::marker::PhantomData;
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{Error, Visitor};

pub fn option_form_vec_deserialize<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
    where D: Deserializer<'de>,
          T: FromStr {
    form_vec_deserialize(deserializer).map(|v| Some(v))
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
        formatter.write_str("form array in GET http request's query parameter")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct Address {
        city: String,
        postcode: String,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct QueryParams {
        id: Option<u32>,
        #[serde(deserialize_with = "form_vec_deserialize")]
        user_ids: Vec<u8>,
        #[serde(deserialize_with = "option_form_vec_deserialize", default)]
        user_names: Option<Vec<String>>,
    }

    #[test]
    fn it_works() {
        let rec_params: QueryParams =
            serde_urlencoded::from_str("id=12345&user_ids=1a,2,3")
            .unwrap();
        println!("{:#?}", rec_params);
    }
}
