use factrs3::{fact, dec_digits};
use std::io::{self, Write};
use std::time::Instant;

/// Formatea un número entero agregando comas como separadores de miles para mejorar la legibilidad.
fn format_thousands(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result
}

fn main() {
    print!("Introduce el número para calcular el factorial [default: 180000]: ");
    let _ = io::stdout().flush(); // Asegurar que el prompt se imprima antes de esperar entrada

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let n: usize = input.trim().parse().unwrap_or(180_000);

    println!("Calculando factorial de {}...", format_thousands(n));
    
    // Medimos únicamente el tiempo de cálculo del factorial
    let start_calc = Instant::now();
    let s = fact(n);
    let duration_calc = start_calc.elapsed();

    // Calculamos el número de dígitos utilizando la función matemática optimizada
    let digits = dec_digits(&s);
    let ms = duration_calc.as_secs_f64() * 1000.0;

    println!(
        "¡El factorial de {} (un número con {} dígitos) se calculó en tan solo {:.1} milisegundos!",
        format_thousands(n),
        format_thousands(digits),
        ms
    );
}
