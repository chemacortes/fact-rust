use num::BigUint;
use rayon::prelude::*;

/// Calcula el factorial de un número n utilizando BigUint para manejar números grandes.
/// Implementa paralelización con Rayon para optimizar el rendimiento en n grandes,
/// utilizando una estrategia de reducción de árbol de multiplicación.
pub fn fact(n: usize) -> BigUint {
    // Para valores pequeños de n, la sobrecarga de spawnear hilos en Rayon
    // supera los beneficios del paralelismo; utilizamos un bucle secuencial.
    if n < 1000 {
        (1..=n).map(BigUint::from).product()
    } else {
        // En n grandes, la reducción de árbol es eficiente, manteniendo
        // los operandos de tamaño similar durante la multiplicación.
        (1..=n)
            .into_par_iter()
            .map(BigUint::from)
            .product()
    }
}
