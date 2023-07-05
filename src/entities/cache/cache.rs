use bincode;
use chrono::{DateTime, Duration, Utc};
use dashmap::{mapref::one::Ref, DashMap};
use log::{info, debug};
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

    fn insert<T>(&self, key: String, value: T, expiry: Option<Duration>) -> Option<T>
    where
        for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
    {
        // TODO: handle error
        let encoded = wrap_and_encode(value, expiry);
        let existing = self.map.insert(key, encoded);

        decode_and_unwrap_value(existing)
    }

    fn get<T>(&self, key: String) -> Option<T>
    where
        for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
    {
        // TODO: handle error
        let encoded = self.map.get(&key);

        decode_and_unwrap_ref(encoded)
    }

    fn remove<T>(&self, key: String) -> Option<T>
    where
        for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
    {
        let encoded = self.map.remove(&key);

        decode_and_unwrap(encoded)
    }

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
        match self.get(key.to_owned()) {
            Some(value) => {
                info!("Got from cache: {:?}", value);
                Ok(value)
            }
            None => {
                info!("Not found in cache: {}", key);
                let source_value = get_source().await?;
                info!("Got from source: {:?}", source_value);
                let keys = keys_builder(&source_value);
                for key in keys {
                    info!("Adding to cache: {} = {:?}", key, source_value);
                    self.insert(key, source_value.clone(), expiry);
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
            self.insert(key, source_value.clone(), expiry);
        }

        Ok(source_value)
    }
}
/*
trait CachableTraitGuard {
    type Mirror: for<'a> Deserialize<'a> + Serialize + PartialEq + Clone;
}

impl<T> CachableTraitGuard for T
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone,
{
    type Mirror = Self;
}

type CacheableValue<T> = <T as CachableTraitGuard>::Mirror;

trait CacheableFutureTraitGuard<T>
{
    type Mirror: Future<Output = Result<CacheValue<T>, EntityError>> + Sized;
}

impl<T, Fut> CacheableFutureTraitGuard<T> for T
    where Fut: Future<Output = Result<CacheValue<T>, EntityError>> + Sized
{
    type Mirror = Self;
}

type CacheableFutureValue<T> = <T as CacheableFutureTraitGuard<T>>::Mirror;

pub type InsertSource<T> = fn() -> CacheableFutureValue<T>;


// type InsertSource<Id> = fn() -> Future<Output = Result<Id, EntityError>>;
*/

fn wrap_and_encode<T>(value: T, expiry: Option<Duration>) -> Vec<u8>
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
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

fn decode_and_unwrap_value<T>(encoded: Option<Vec<u8>>) -> Option<T>
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
{
    match encoded {
        Some(encoded) => do_decode_and_unwrap(&encoded),
        None => None,
    }
}

fn decode_and_unwrap<T>(encoded: Option<(String, Vec<u8>)>) -> Option<T>
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
{
    match encoded {
        Some((_, encoded)) => do_decode_and_unwrap(&encoded),
        None => None,
    }
}

fn decode_and_unwrap_ref<T>(encoded: Option<Ref<String, Vec<u8>>>) -> Option<T>
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
{
    match encoded {
        Some(encoded) => do_decode_and_unwrap(encoded.value()),
        None => None,
    }
}

fn do_decode_and_unwrap<T>(encoded: &Vec<u8>) -> Option<T>
where
    for<'a> T: Deserialize<'a> + Serialize + PartialEq + Clone + std::fmt::Debug,
{
    let wrapped: CacheValue<T> = bincode::deserialize(&encoded[..]).unwrap();
    debug!("Got decoded value: {:?}", wrapped);
    match wrapped.expiry {
        Some(expiry) => {
            info!("Got expiry: {:?}", expiry);
            if expiry > Utc::now() {
                info!("Not expired");
                Some(wrapped.value)
            } else {
                info!("Expired");
                None
            }
        }
        None => {
            Some(wrapped.value)
        }
    }
}
