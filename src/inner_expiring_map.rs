use std::time::{SystemTime, Duration};
use std::collections::HashMap;
use std::hash::Hash;
use std::borrow::Borrow;
use std::ops::Add;

struct ValueContainer<V> {
    value: V,
    expire_time: SystemTime,
}

pub(crate) struct InnerExpiringMap<K, V> {
    inner: HashMap<K, ValueContainer<V>>,
    time_to_live: Duration,
}

impl<V> ValueContainer<V> {
    fn new(value: V, expire_time: SystemTime) -> Self {
        ValueContainer {
            value,
            expire_time,
        }
    }
}

impl<K, V> InnerExpiringMap<K, V>
    where K: Eq + Hash
{
    pub(crate) fn new(time_to_live: Duration) -> Self {
        InnerExpiringMap {
            inner: HashMap::new(),
            time_to_live,
        }
    }

    pub(crate) fn insert(&mut self, k: K, v: V, current_time: SystemTime) -> Option<V> {
        let value_container =
            ValueContainer::new(v, current_time.add(self.time_to_live));

        self.inner.insert(k, value_container)
            .and_then(|val_container| {
                if current_time.le(&val_container.expire_time) {
                    // only return the previous value if it was not expired
                    Some(val_container.value)
                } else {
                    None
                }
            })
    }

    pub(crate) fn get<Q: ?Sized>(&mut self, k: &Q, current_time: SystemTime) -> Option<&V>
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        self.inner.get(k).and_then(|val_container| {
            if current_time.le(&val_container.expire_time) {
                Some(&val_container.value)
            } else {
                None
            }
        })
    }

    pub(crate) fn get_mut<Q: ?Sized>(&mut self, k: &Q, current_time: SystemTime) -> Option<&mut V>
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        self.inner.get_mut(k).and_then(|val_container| {
            if current_time.le(&val_container.expire_time) {
                Some(&mut val_container.value)
            } else {
                None
            }
        })
    }

    pub(crate) fn remove_expired_entries(&mut self, current_time: SystemTime) {
        self.inner.retain(|_k, v| current_time.le(&v.expire_time));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_map() -> InnerExpiringMap<String, String> {
        let time_to_live = Duration::from_secs(60);

        InnerExpiringMap::new(time_to_live)
    }

    #[test]
    fn insert_and_get() {
        let mut map = get_test_map();

        map.insert("keyA".to_owned(), "valA".to_owned(), SystemTime::now());

        assert_eq!(Some(&mut "valA".to_owned()), map.get_mut("keyA", SystemTime::now()));
        assert_eq!(Some(&"valA".to_owned()), map.get("keyA", SystemTime::now()));
    }

    #[test]
    fn entry_expires_after_time_to_live() {
        let mut map : InnerExpiringMap<String, String> = get_test_map();

        map.insert("keyA".to_owned(), "valA".to_owned(), SystemTime::now());

        let read_time = SystemTime::now().add(Duration::from_secs(30));
        assert_eq!(Some(&mut "valA".to_owned()), map.get_mut("keyA", read_time));
        assert_eq!(Some(&"valA".to_owned()), map.get("keyA", read_time));

        let read_time_2 = SystemTime::now().add(Duration::from_secs(65));
        assert_eq!(None, map.get_mut("keyA", read_time_2));
        assert_eq!(None, map.get("keyA", read_time_2));
    }

    #[test]
    fn remove_expired_entries() {
        let mut map : InnerExpiringMap<String, String> = get_test_map();

        map.insert("keyA".to_owned(), "valA".to_owned(), SystemTime::now());

        assert_eq!(1, map.inner.len());

        map.remove_expired_entries(SystemTime::now().add(Duration::from_secs(65)));

        assert_eq!(0, map.inner.len());
    }

}
