//! Dans ce fichier, je propose une implementation d’un cache LRU stocke en memoire.
//!
//!
//! - Je m’appuie sur une `HashMap` pour faire la correspondance (cle vers valeur).
//! - Je maintiens un `Vec<K>` pour garder la trace de l’ordre d’utilisation :
//!   - La position `0` dans le `Vec` sera la plus ancienne entree.
//!   - La fin du `Vec` (index `n-1`) sera la plus recente.
//! - Chaque fois que j’accède à une cle (ac els verbes `get` ou `put`), je la deplace en fin de `Vec`.
//! - Quand le cache est plein et que j’ajoute une nouvelle cle, j’ejecte la cle la moins recente
//!   (celle qui est à l’index 0 de `usage_order`).

use std::collections::HashMap;
use std::hash::Hash;

use super::LRUCache;

/// `EphemeralCache` me sert à gerer un cache LRU 100% en memoire. ( j'ai eu des prblms avec la persistance donc pour l'instant j'ai qu'un projet ac les tests en memoire)
///
/// - `capacity` : c’est la taille max que je souhaite autoriser.
/// - `store` : c’est ma `HashMap` qui map les cles sur leurs valeurs.
/// - `usage_order` : je l’utilise pour savoir qui est le plus ancien et qui est le plus recent.
#[derive(Debug)]
pub struct EphemeralCache<K, V> {
    capacity: usize,
    store: HashMap<K, V>,
    usage_order: Vec<K>,
}

impl<K, V> EphemeralCache<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Je construis un nouveau `EphemeralCache` en indiquant la capacite maximale.
    ///
    /// # Exemple
    ///
    /// ```
    /// use eval_rust_last_v::lru::ephemeral::EphemeralCache;
    /// use eval_rust_last_v::lru::LRUCache;
    ///
    /// let cache = EphemeralCache::<i32, String>::new(3);
    /// assert_eq!(cache.capacity(), 3);
    /// ```
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            store: HashMap::new(),
            usage_order: Vec::new(),
        }
    }

    /// Ici, je declare `key` comme la plus recemment utilisee,
    /// en la deplaçant à la fin de `usage_order`.
    fn mark_as_recent(&mut self, key: &K) {
        // Je verifie d’abord si la cle est dejà presente quelque part dans usage_order...
        if let Some(pos) = self.usage_order.iter().position(|k| k == key) {
            //et si oui, je l’enlève de cette position.
            self.usage_order.remove(pos);
        }
        // Puis je la pousse à la findu tableau : c’est mtn la plus recente.
        self.usage_order.push(key.clone());
    }
}

/// J'utilise mtn le trait `LRUCache` pour `EphemeralCache`.
impl<K, V> LRUCache<K, V> for EphemeralCache<K, V>
where
    K: Eq + Hash + Clone,
{
    fn put(&mut self, key: K, value: V) {
        // Si la cle existe dejà, je mets à jour la valeur et la declare "recente".
        if self.store.contains_key(&key) {
            self.store.insert(key.clone(), value);
            self.mark_as_recent(&key);
        } else {
            // Si on est plein, j’ejecte d’abord la cle la moins recente (index 0).
            if self.store.len() == self.capacity {
                if let Some(least_recent) = self.usage_order.first() {
                    self.store.remove(least_recent);
                }
                self.usage_order.remove(0);
            }
            // J’insère la nouvelle cle, puis je la marque "recente".
            self.store.insert(key.clone(), value);
            self.usage_order.push(key);
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        // Si la cle est dans le store, je la marque comme plus recente...
        if self.store.contains_key(key) {
            self.mark_as_recent(key);
            self.store.get(key)
        } else {
            None
        }
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

// -- Tests U --
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lru::LRUCache;

    #[test]
    fn test_lru_simple() {
        let mut cache = EphemeralCache::new(2);

        // Je commence par inserer A et B
        cache.put("A", 1);
        cache.put("B", 2);
        println!("[TEST] Après put(A=1), put(B=2) => {:#?}", cache);

        // J’accède à A, donc A devient plus recent
        assert_eq!(cache.get(&"A"), Some(&1));
        println!("[TEST] Après get(A) => {:#?}", cache);

        // J’accède à B, donc B devient plus recent
        assert_eq!(cache.get(&"B"), Some(&2));
        println!("[TEST] Après get(B) => {:#?}", cache);

        // J’insère C, et comme je suis plein (capacite = 2), j’ejecte le plus ancien (A)
        cache.put("C", 3);
        println!("[TEST] Après put(C=3) => {:#?}", cache);

        // Je m’assure que A est parti, et que B et C sont toujours là
        assert_eq!(cache.get(&"A"), None);
        assert_eq!(cache.get(&"B"), Some(&2));
        assert_eq!(cache.get(&"C"), Some(&3));

        println!("[TEST] Fin du test => {:#?}", cache);
    }
}
