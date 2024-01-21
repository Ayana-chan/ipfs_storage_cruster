use std::marker::PhantomData;
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{Error, Visitor};

fn opt_vec_de<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
    where D: Deserializer<'de>,
          T: Deserialize<'de> + FromStr + Default {
    vec_de(deserializer).map(|v| Some(v))
}

fn vec_de<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where D: Deserializer<'de>,
          T: Deserialize<'de> + FromStr + Default {
    deserializer.deserialize_str(MyVecVisitor::<T>::new())
}

struct MyVecVisitor<T> {
    marker: PhantomData<fn() -> Vec<T>>,
}

impl<T> MyVecVisitor<T> {
    fn new() -> Self {
        Self {
            marker: PhantomData
        }
    }
}

impl<'de, T> Visitor<'de> for MyVecVisitor<T>
    where
        T: Deserialize<'de> + FromStr + Default
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
            .map(|s| {
                // value: Custom("invalid type: string \"1\", expected u8")
                // String is ok, but u8 is not. It seems that this method has not actually undergone type conversion
                // let strdes: StrDeserializer<E> = s.into_deserializer();
                // let ans = T::deserialize::<_>(strdes).unwrap();
                // ans
                T::from_str(s).unwrap_or_default()
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
        address: Address,
        #[serde(deserialize_with = "vec_de")]
        user_ids: Vec<u8>,
        #[serde(deserialize_with = "opt_vec_de", default)]
        user_names: Option<Vec<String>>,
    }

    #[test]
    fn it_works() {
        // let rec_params: QueryParams = serde_qs::from_str("\
        // address[postcode]=12345&address[city]=Carrot+City&\
        // user_ids=1,2")
        //     .unwrap();
        let rec_params: QueryParams = serde_qs::from_str("\
    id=114514&address[postcode]=12345&address[city]=Carrot+City&\
    user_ids=1,2&user_names=yuuka,ayana,mika")
            .unwrap();
        println!("{:#?}", rec_params);
    }
}
