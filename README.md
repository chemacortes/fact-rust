# factrs3 🚀

**factrs3** es una herramienta y biblioteca en Rust ultra-eficiente diseñada para el cálculo y análisis de factoriales de números extremadamente grandes (cálculo de precisión arbitraria).

El proyecto combina computación paralela a nivel de CPU con algoritmos matemáticos avanzados para realizar cálculos masivos en fracciones de segundo.

---

## ✨ Características Principales

1. **Paralelización con Rayon**: Divide el rango de multiplicaciones en subproblemas balanceados utilizando hilos a nivel de hardware, logrando tiempos de ejecución sub-segundo incluso para factoriales de cientos de miles.
2. **Aritmética de Precisión Arbitraria**: Integra `num::BigUint` de la suite `num` para manejar números con millones de dígitos sin desbordamientos de memoria.
3. **Cálculo de Dígitos Ultra-Rápido ($O(1)$ amortizado)**: En lugar de convertir los números gigantescos a cadenas de texto decimales (`to_string().len()`) —lo cual requiere divisiones sucesivas costosas $O(N^2)$—, el proyecto implementa un método propio analítico:
   * Calcula la cota inferior estimada de dígitos a partir del número de bits del número ($s.\text{bits()}$).
   * Compara el número exacto de bits de $s$ contra la potencia límite $10^k$ calculada mediante su fórmula analítica de bits: $\text{bits}(10^k) = k + \lfloor k \cdot \log_2(5) \rfloor + 1$.
   * Evita realizar exponenciaciones y divisiones pesadas en más del **99.9% de los casos**, reduciendo el cálculo de segundos a microsegundos.
4. **Formateo de Salida Premium**: La interfaz CLI muestra los resultados de forma elegante con separadores de miles y milisegundos de precisión.

---

## ⚡ Rendimiento en Acción (`n = 180,000`)

Cálculo del factorial de $180,000$ (cuyo resultado tiene **867,780 dígitos** decimales):

* **Tiempo de Cálculo del Factorial**: **~82.6 ms** 🚀
* **Tiempo para Contar los Dígitos Exactos (Método Optimizado)**: **~43.4 ms** (en comparación con **>2,000 ms** utilizando el método tradicional `to_string().len()`).

---

## 🛠️ Requisitos e Instalación

Asegúrate de tener instalado el toolchain de Rust. Puedes comprobarlo ejecutando:
```bash
rustc --version
```

### Clonar y Compilar

1. Compila el proyecto en modo release para habilitar las optimizaciones de compilador más potentes:
   ```bash
   cargo build --release
   ```

2. Ejecuta el binario directamente:
   ```bash
   cargo run --release
   ```

---

## 📂 Estructura del Código

El proyecto está diseñado bajo un enfoque modular limpio que separa la lógica pura de la interfaz de usuario:

* **[`src/lib.rs`](src/lib.rs)**: Contiene el núcleo matemático y los algoritmos optimizados de biblioteca. Expone `fact(n)` para el cálculo del factorial y `dec_digits(s)` para obtener la cantidad exacta de dígitos.
* **[`src/main.rs`](src/main.rs)**: Punto de entrada de la CLI, lectura de datos interactiva del usuario y presentación elegante del rendimiento medido.

---

## 📜 Licencia

Este proyecto está distribuido bajo la licencia MIT. Siéntete libre de usarlo, modificarlo y compartirlo.
