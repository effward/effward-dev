use bincode;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, future::Future};

use crate::entities::EntityError;

#[derive(Clone, Debug)]
pub struct Cache {
    map: HashMap<String, Vec<u8>>,
}

#[derive(Deserialize, Serialize, PartialEq)]
struct CacheValue<T> {
    pub value: T,
    pub expiry: Option<DateTime<Utc>>,
}

impl Cache {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }
    
    fn insert<T>(&mut self, key: String, value: T, expiry: Option<Duration>) -> Option<T>
    where
        for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone,
    {
        // TODO: handle error
        let encoded = wrap_and_encode(value, expiry);
        let existing = self.map.insert(key, encoded);

        decode_and_unwrap(existing)
    }

    fn get<T>(&self, key: String) -> Option<T>
    where
        for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone,
    {
        // TODO: handle error
        let encoded = self.map.get(&key);

        decode_and_unwrap_ref(encoded)
    }

    fn remove<T>(&mut self, key: String) -> Option<T>
    where
        for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone,
    {
        let encoded = self.map.remove(&key);

        decode_and_unwrap(encoded)
    }

    pub async fn get_cached<T, Fut>(
        &mut self,
        key: String,
        expiry: Option<Duration>,
        get_source: fn() -> Fut,
    ) -> Result<T, EntityError>
    where
        for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone,
        Fut: Future<Output = Result<T, EntityError>> + 'static,
    {
        match self.get(key.to_owned()) {
            Some(value) => Ok(value),
            None => {
                let source_value = get_source().await?;
                self.insert(key, source_value.to_owned(), expiry);
                Ok(source_value)
            }
        }
    }

    pub async fn insert_cached<T, Id, Fut>(
        &mut self,
        key: String,
        value: T,
        expiry: Option<Duration>,
        write_source: fn() -> Fut,
    ) -> Result<Id, EntityError>
    where
        for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone,
        Fut: Future<Output = Result<Id, EntityError>> + 'static,
    {
        let source_id = write_source().await?;

        self.insert(key, value, expiry);

        Ok(source_id)
    }
}

fn wrap_and_encode<T>(value: T, expiry: Option<Duration>) -> Vec<u8>
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone,
{
    let deadline = match expiry {
        Some(expiry) => Some(Utc::now() + expiry),
        None => None,
    };
    let wrapped_value = CacheValue {
        value,
        expiry: deadline,
    };

    bincode::serialize(&wrapped_value).unwrap()
}

fn decode_and_unwrap<T>(encoded: Option<Vec<u8>>) -> Option<T>
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone,
{
    match encoded {
        Some(encoded) => do_decode_and_unwrap(&encoded),
        None => None,
    }
}

fn decode_and_unwrap_ref<T>(encoded: Option<&Vec<u8>>) -> Option<T>
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone,
{
    match encoded {
        Some(encoded) => do_decode_and_unwrap(encoded),
        None => None,
    }
}

fn do_decode_and_unwrap<T>(encoded: &Vec<u8>) -> Option<T>
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone,
{
    let wrapped: CacheValue<T> = bincode::deserialize(&encoded[..]).unwrap();
    match wrapped.expiry {
        Some(expiry) => {
            if expiry > Utc::now() {
                Some(wrapped.value)
            } else {
                None
            }
        }
        None => Some(wrapped.value),
    }
}
