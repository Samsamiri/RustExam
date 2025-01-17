//! Ici, je definis l’archi generale pour gerer un cache LRU.
//!

use std::hash::Hash;

/// Ce trait represente la façon dont je vois un cache LRU generique.
///
/// - `K` represente le type de la cle (je demande `Eq + Hash + Clone` pour pouvoir gerer un HashMap et la manipuler facilement).
/// - `V` represente la valeur stockee.
pub trait LRUCache<K, V>
where
    K: Eq + Hash + Clone,
{
    /// J’insère ou je mets à jour la valeur associee à `key`.
    ///
    /// - Si la cle existe dejà, je mets à jour sa valeur et je la declare
    ///   comme la plus recemment utilisee.
    /// - Si la cle n’existe pas et que le cache est plein, j’ejecte la cle
    ///   la moins recemment utilisee pour faire de la place.
    fn put(&mut self, key: K, value: V);

    /// Je recupère la valeur associee à `key`, si elle est presente dans le cache.
    /// Si c’est le cas, je marque `key` comme la plus recemment utilisee.
    fn get(&mut self, key: &K) -> Option<&V>;

    /// Je renvoie la capacite maximale de mon cache, cad le nombre
    /// d’entrees maximum.
    fn capacity(&self) -> usize;
}

pub mod ephemeral;
