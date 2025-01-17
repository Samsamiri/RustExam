use eval_rust_last_v::lru::{LRUCache, ephemeral::EphemeralCache};

fn main() {
    println!("=== DEMO CCACHE LRU ===");

    let mut cache = EphemeralCache::new(2);
    println!("[MAIN] Capacite du cache = {}, tableau initial = [ ] (vide)", cache.capacity());
    println!("[MAIN] Etat du cache : {:#?}\n", cache);

    // Insertion de A = 10
    cache.put("A", 10);
    println!("[MAIN] Je viens d’inserer A=10, donc tableau = [A]. A est le plus recent.");
    println!("[MAIN] Etat du cache après put(A,10) : {:#?}\n", cache);

    // Insertion de B = 20
    cache.put("B", 20);
    println!("[MAIN] Je viens d’inserer B=20, tableau = [A, B]. B est le plus recent.");
    println!("[MAIN] Etat du cache après put(B,20) : {:#?}\n", cache);

    // get(A) => A devient plus recent
    let val_a = cache.get(&"A");  // Emprunt mutable ici
    println!("[MAIN] get(A) => {:?}", val_a);
    // Fin de la ligne => on libère val_a
    println!("[MAIN] Maintenant, A est le plus recent, donc tableau = [B, A].");
    println!("[MAIN] Etat du cache après get(A) : {:#?}\n", cache);

    // get(B) => B devient plus recent
    let val_b = cache.get(&"B");  // Emprunt mutable
    println!("[MAIN] get(B) => {:?}", val_b);
    // Fin de la ligne => on libère val_b
    println!("[MAIN] Maintenant, B est le plus recent, donc tableau = [A, B].");
    println!("[MAIN] Etat du cache après get(B) : {:#?}\n", cache);

    // Insertion de C => on ejecte le plus ancien (ici, A)
    cache.put("C", 30);
    println!("[MAIN] J’insère C=30, donc je suis plein, j’ejecte A, tableau = [B, C].");
    println!("[MAIN] Etat du cache après put(C,30) : {:#?}\n", cache);

    let val_a_after = cache.get(&"A");
    println!("[MAIN] get(A) => {:?}, (A devrait être None, car ejecte)", val_a_after);

    let val_b_after = cache.get(&"B");
    println!("[MAIN] get(B) => {:?}", val_b_after);

    let val_c_after = cache.get(&"C");
    println!("[MAIN] get(C) => {:?}", val_c_after);

    println!("\n[MAIN] Cache final : {:#?}", cache);
    println!("=== FIN DE LA DEMO ===");
}
