use num::BigUint;
use rayon::prelude::*;

/// Calcula el factorial de un número n utilizando paralelización con Rayon.
pub fn fact(n: usize) -> BigUint {
    (1..=n).into_par_iter().map(BigUint::from).product()
}

/// Calcula exactamente la cantidad de dígitos decimales de un BigUint de forma altamente eficiente,
/// evitando la conversión a cadena de caracteres (String) en la gran mayoría de los casos.
pub fn dec_digits(s: &BigUint) -> usize {
    let bits = s.bits();
    if bits == 0 {
        return 1;
    }
    
    // d_low es una cota inferior para el número de dígitos - 1
    let d_low = ((bits - 1) as f64 * std::f64::consts::LOG10_2) as usize;
    let k = d_low + 1; // Exponente a comprobar: 10^k
    
    // Calculamos el número exacto de bits de 10^k usando la fórmula:
    // bits(10^k) = k + floor(k * log2(5)) + 1
    let log2_5 = 2.3219280948873626_f64;
    let limit_bits = k as u64 + (k as f64 * log2_5).floor() as u64 + 1;
    
    if bits < limit_bits {
        d_low + 1
    } else if bits > limit_bits {
        d_low + 2
    } else {
        // En el caso extremadamente raro de que tengan el mismo número de bits,
        // construimos 10^k para hacer la comparación exacta.
        let limit = BigUint::from(10u32).pow(k as u32);
        if s >= &limit {
            d_low + 2
        } else {
            d_low + 1
        }
    }
}


