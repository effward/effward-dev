use bincode;
use chrono::{DateTime, Duration, Utc};
use dashmap::{mapref::one::Ref, DashMap};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::future::Future;

use crate::entities::EntityError;

#[derive(Clone, Debug)]
pub struct Cache {
    map: DashMap<String, Vec<u8>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct CacheValue<T> {
    pub value: T,
    pub expiry: Option<DateTime<Utc>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            map: DashMap::new(),
        }
    }

    fn insert<T>(
        &self,
        key: String,
        value: T,
        expiry: Option<Duration>,
    ) -> Result<Option<T>, EntityError>
    where
        for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
    {
        // TODO: handle error
        let encoded = wrap_and_encode(value, expiry)?;
        let existing = self.map.insert(key, encoded);

        decode_and_unwrap_value(existing)
    }

    fn get<T>(&self, key: String) -> Result<Option<T>, EntityError>
    where
        for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
    {
        // TODO: handle error
        let encoded = self.map.get(&key);

        decode_and_unwrap_ref(encoded)
    }

    // TODO: use in invalidation
    // fn remove<T>(&self, key: String) -> Result<Option<T>, EntityError>
    // where
    //     for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
    // {
    //     let encoded = self.map.remove(&key);

    //     decode_and_unwrap(encoded)
    // }

    // TODO: expand key builder to allow for tags/collections
    pub async fn get_cached<T, Fut, F, FKey>(
        &self,
        key: String,
        get_source: F,
        keys_builder: FKey,
        expiry: Option<Duration>,
    ) -> Result<T, EntityError>
    where
        for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
        Fut: Future<Output = Result<T, EntityError>> + Sized,
        F: FnOnce() -> Fut,
        FKey: FnOnce(&T) -> Vec<String>,
    {
        info!("Getting: {}", key);
        match self.get(key.to_owned())? {
            Some(value) => {
                info!("Got from cache: {:?}", value);
                Ok(value)
            }
            None => {
                let source_value = get_source().await?;
                info!("Got from source: {:?}", source_value);
                let keys = keys_builder(&source_value);
                for key in keys {
                    info!("Adding to cache: {} = {:?}", key, source_value);
                    match self.insert(key, source_value.clone(), expiry) {
                        Ok(_) => (),
                        Err(e) => error!("Error adding value to cache. Error: {:?}", e),
                    }
                }

                Ok(source_value)
            }
        }
    }

    pub async fn insert_cached<T, Fut, F>(
        &self,
        insert_source: F,
        keys_builder: fn(&T) -> Vec<String>,
        expiry: Option<Duration>,
    ) -> Result<T, EntityError>
    where
        for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
        Fut: Future<Output = Result<T, EntityError>> + Sized,
        F: FnOnce() -> Fut,
    {
        let source_value = insert_source().await?;
        let keys = keys_builder(&source_value);
        for key in keys {
            match self.insert(key, source_value.clone(), expiry) {
                Ok(_) => (),
                Err(e) => error!("Error adding value to cache. Error: {:?}", e),
            }
        }

        Ok(source_value)
    }
}

fn wrap_and_encode<T>(value: T, expiry: Option<Duration>) -> Result<Vec<u8>, EntityError>
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
{
    let deadline = expiry.map(|expiry| Utc::now() + expiry);
    let wrapped_value = CacheValue {
        value,
        expiry: deadline,
    };

    match bincode::serialize(&wrapped_value) {
        Ok(encoded) => Ok(encoded),
        Err(e) => Err(EntityError::CachingError(e.to_string())),
    }
}

fn decode_and_unwrap_value<T>(encoded: Option<Vec<u8>>) -> Result<Option<T>, EntityError>
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
{
    match encoded {
        Some(encoded) => do_decode_and_unwrap(&encoded),
        None => Ok(None),
    }
}

// fn decode_and_unwrap<T>(encoded: Option<(String, Vec<u8>)>) -> Result<Option<T>, EntityError>
// where
//     for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
// {
//     match encoded {
//         Some((_, encoded)) => do_decode_and_unwrap(&encoded),
//         None => Ok(None),
//     }
// }

fn decode_and_unwrap_ref<T>(encoded: Option<Ref<String, Vec<u8>>>) -> Result<Option<T>, EntityError>
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
{
    match encoded {
        Some(encoded) => do_decode_and_unwrap(encoded.value()),
        None => Ok(None),
    }
}

fn do_decode_and_unwrap<T>(encoded: &[u8]) -> Result<Option<T>, EntityError>
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
{
    let wrapped: CacheValue<T> = match bincode::deserialize(encoded) {
        Ok(decoded) => decoded,
        Err(e) => {
            return Err(EntityError::CachingError(e.to_string()));
        }
    };
    debug!("Got decoded value: {:?}", wrapped);
    match wrapped.expiry {
        Some(expiry) => {
            info!("Got expiry: {:?}", expiry);
            if expiry > Utc::now() {
                info!("Not expired");
                Ok(Some(wrapped.value))
            } else {
                info!("Expired");
                Ok(None)
            }
        }
        None => Ok(Some(wrapped.value)),
    }
}
