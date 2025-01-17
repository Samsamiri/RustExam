# Projet Rust – Cache LRU

Ce projet implémente un **cache LRU** (Least Recently Used) en Rust, dans sa version éphémère (stockage en mémoire).

## Description

- **LRUCache (trait)**  
  Définit les opérations essentielles d’un cache LRU :  
  - **put** : mettre à jour ou insérer une clé et sa valeur  
  - **get** : récupérer la valeur d’une clé (en la marquant plus récemment utilisée)  
  - **capacity** : renvoyer la capacité du cache  

- **Implémentation éphémère**  
  Conserve uniquement en mémoire les éléments les plus récemment utilisés.  
  Éjecte automatiquement la donnée la moins récente lorsque le cache est plein.

- **Test unitaire**  
  Vérifie le bon fonctionnement de l’implémentation (insertion, accès, éjection).

- **`main.rs`**  
  Fournit un exemple d’utilisation et des logs détaillés sur les opérations effectuées (put, get, éjection).

## Principe de fonctionnement du Cache LRU

1. **Insertion (put)**  
   - Si le cache n’est pas plein, j’insère simplement la nouvelle donnée.  
   - Si le cache est plein, je retire la clé la moins récemment utilisée avant d’insérer la nouvelle donnée.

2. **Accès (get)**  
   - Chaque accès à une clé la déclare comme “plus récemment utilisée”.  
   - La clé est donc déplacée en fin de liste (position la plus récente).

3. **Éjection**  
   - Lorsque la taille maximale du cache est atteinte, l’insertion d’une nouvelle clé entraîne l’éjection de la plus ancienne.  
   - Celle-ci est retirée du stockage, (ou considérée comme `None`).

## Comment exécuter le projet

1. **Compilation et exécution**

     ```bash
     cargo build
     cargo run
2. **Tests**
     
```bash
cargo test
```
Cela exécute le test unitaire qui vérifie le comportement LRU.

## Organisation du code

- **`src/lib.rs`** : Point d’entrée de la bibliothèque  
- **`lru/mod.rs`** : Définition du trait `LRUCache` et réexport des sous-modules  
- **`lru/ephemeral.rs`** : Implémentation éphémère du cache (en mémoire)  
- **`main.rs`** : Exemple d’utilisation et logs détaillés  
- *(Autres fichiers de config : `Cargo.toml`, etc.)*

