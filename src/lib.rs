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
        if s >= &limit { d_low + 2 } else { d_low + 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Comprueba que `dec_digits` coincide con la referencia exacta (`to_string().len()`).
    fn assert_digits(v: &BigUint) {
        let expected = v.to_string().len();
        assert_eq!(dec_digits(v), expected, "valor: {}", v);
    }

    #[test]
    fn dec_digits_zero_and_one() {
        assert_eq!(dec_digits(&BigUint::from(0u32)), 1);
        assert_eq!(dec_digits(&BigUint::from(1u32)), 1);
    }

    #[test]
    fn dec_digits_powers_of_ten() {
        // 10^0 .. 10^40: ejercita la rama de comparación exacta contra 10^k.
        for k in 0..=40u32 {
            assert_digits(&BigUint::from(10u32).pow(k));
        }
    }

    #[test]
    fn dec_digits_around_powers_of_ten() {
        // Valores inmediatamente anterior, igual y posterior a cada potencia de 10.
        for k in 1..=20u32 {
            let p = BigUint::from(10u32).pow(k);
            let one = BigUint::from(1u32);
            assert_digits(&(&p - &one));
            assert_digits(&p);
            assert_digits(&(&p + &one));
        }
    }

    #[test]
    fn dec_digits_bit_boundaries() {
        // Fronteras de bits: donde la cota baja (bits - 1) cambia de valor.
        let values: &[u64] = &[
            2,
            3,
            4,
            7,
            8,
            15,
            16,
            255,
            256,
            65_535,
            65_536,
            (1 << 32) - 1,
            1 << 32,
            (1 << 32) + 1,
            (1 << 63) - 1,
            1 << 63,
            u64::MAX,
        ];
        for &v in values {
            assert_digits(&BigUint::from(v));
        }
    }

    #[test]
    fn dec_digits_large_factorial() {
        let s = fact(1000);
        assert_digits(&s);
    }

    #[test]
    fn fact_small_values() {
        assert_eq!(fact(0), BigUint::from(1u32));
        assert_eq!(fact(1), BigUint::from(1u32));
        assert_eq!(fact(2), BigUint::from(2u32));
        assert_eq!(fact(5), BigUint::from(120u32));
        assert_eq!(fact(10), BigUint::from(3_628_800u32));
    }
}
