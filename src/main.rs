use factrs3::fact;
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    print!("Introduce el número para calcular el factorial [default: 180000]: ");
    let _ = io::stdout().flush(); // Asegurar que el prompt se imprima antes de esperar

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer entrada");
    
    let n: usize = input.trim().parse().unwrap_or(180_000);
    
    println!("Calculando factorial de {}...", n);
    
    // Medimos el tiempo de ejecución para evaluar la eficiencia del paralelismo.
    let start = Instant::now();
    let s = fact(n);
    let duration = start.elapsed();

    println!("Factorial calculado en: {:?}", duration);
    // Imprimimos la cantidad de dígitos, que es una métrica útil para números gigantes.
    println!("Cantidad de dígitos en el resultado: {}", s.to_string().len());
}
