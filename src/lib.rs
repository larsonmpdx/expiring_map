mod inner_expiring_map;

use inner_expiring_map::InnerExpiringMap;
use std::time::Duration;
use std::hash::Hash;
use std::time::SystemTime;
use std::borrow::Borrow;

pub struct ExpiringMap<K, V> {
    inner: InnerExpiringMap<K, V>
}

impl<K, V> ExpiringMap<K, V>
    where K: Eq + Hash
{
    pub fn new(time_to_live: Duration) -> Self {
        ExpiringMap {
            inner: InnerExpiringMap::new(time_to_live)
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.inner.insert(k, v, SystemTime::now())
    }

    pub fn get<Q: ?Sized>(&mut self, k: &Q) -> Option<&V>
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        self.inner.get(k, SystemTime::now())
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
        where K: Borrow<Q>,
              Q: Hash + Eq
    {
        self.inner.get_mut(k, SystemTime::now())
    }

    pub fn remove_expired_entries(&mut self) {
        self.inner.remove_expired_entries(SystemTime::now());
    }

    pub fn remove(&mut self, k: K) {
        self.inner.remove(k);
    }
}
