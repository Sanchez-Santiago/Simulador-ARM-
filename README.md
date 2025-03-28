# Simulador ARM en Rust

## Descripción
Este proyecto es un simulador simplificado de la arquitectura ARM, implementado en Rust y basado en el módulo Verilog `arm_io_port.v`. El simulador se diseñó para emular una máquina de instrucciones ARM con un conjunto limitado de operaciones aritméticas y de entrada/salida, ejecutándose en un único ciclo. Está orientado a replicar secuencias específicas, tales como "corazón", "rayo láser", "choque" y "auto", y utiliza el registro auxiliar `R7` para manejar operaciones de incremento y decremento de valores. Cabe destacar que no se permite modificar la estructura de la placa ni utilizar instrucciones como `BX`, `CMP`, `MOV` y `BL`.

## Cómo Funciona el Simulador
El simulador opera en varias capas:
- **Interpretación de Instrucciones:**  
  El módulo `traductor.rs` se encarga de leer las instrucciones binarias. Estas se traducen a operaciones aritméticas y de E/S que la placa simula.
- **Ejecución en la Placa ARM:**  
  La estructura `PlacaARM` representa la placa y contiene todos los componentes necesarios para la simulación. Aquí se gestionan registros, la memoria y la lógica de control.
- **Operaciones Aritméticas y de E/S:**  
  La clase `Operacion` implementa las operaciones básicas. Cada instrucción se descompone en una o más operaciones que se ejecutan en un solo ciclo, manteniendo la sincronización y el flujo de datos.
- **Uso del Registro Auxiliar:**  
  Se emplea el registro `R7` como auxiliar para operaciones específicas, principalmente para incrementar y decrementar valores, facilitando la ejecución de secuencias predeterminadas.
- **Integración con Verilog:**  
  Aunque el núcleo del simulador está en Rust, se integra con el módulo Verilog `arm_io_port.v`, el cual maneja la entrada/salida de datos en tiempo real, asegurando que la simulación mantenga un comportamiento coherente con una implementación hardware simplificada.

## Requisitos
- [Rust](https://www.rust-lang.org/) y [Cargo](https://doc.rust-lang.org/cargo/) para compilar y ejecutar el proyecto.
- Conocimientos básicos de arquitectura ARM y Verilog para comprender en detalle el funcionamiento del módulo `arm_io_port.v`.

## Instalación
1. Clona el repositorio:
   ```bash
   git clone https://github.com/tu_usuario/simulador-arm.git

2. Entra en el directorio del proyecto:
   ```bash
   cd simulador-arm
   ```
3. Compila el proyecto:
   ```bash
   cargo build --release
   ```

## Uso
Para ejecutar el simulador, utiliza el siguiente comando (ajusta los parámetros según tus necesidades):
```bash
cargo run --release
```
El simulador interpretará las instrucciones binarias definidas en `traductor.rs`, realizará la conversión a operaciones internas y ejecutará cada instrucción en un ciclo único en la `PlacaARM`.

## Arquitectura del Proyecto
- **PlacaARM:**  
  Representa la placa de hardware simulada, gestionando registros, memoria y la lógica de control.
- **Operacion:**  
  Implementa operaciones aritméticas y de entrada/salida, desglosando cada instrucción en pasos ejecutables en un solo ciclo.
- **traductor.rs:**  
  Es el módulo encargado de interpretar las instrucciones binarias y traducirlas en operaciones que la `PlacaARM` puede ejecutar.
- **arm_io_port.v:**  
  Módulo Verilog que provee la funcionalidad de entrada/salida, asegurando la sincronización y el flujo de datos en un único ciclo.

## Instrucciones Soportadas
El simulador ejecuta un conjunto limitado de instrucciones ARM, adaptadas a la arquitectura simplificada:
- **Operaciones Aritméticas:**  
  Suma, resta y otras operaciones básicas.
- **Secuencias Especiales:**  
  Implementación de secuencias como "corazón", "rayo láser", "choque" y "auto".
- **Operaciones de Control:**  
  Uso del registro `R7` para gestionar operaciones de incremento y decremento.
  
> **Nota:**  
> No se permiten instrucciones como `BX`, `CMP`, `MOV` y `BL`, ni modificaciones en la estructura de la placa.

## Ejemplo de Código
A continuación, se muestra un ejemplo de cómo se estructuran y ejecutan las instrucciones:
```rust
// Ejemplo de creación de una operación aritmética
let operacion = Operacion::new("suma", 5, 3);
let resultado = operacion.ejecutar();
println!("El resultado de la operación es: {}", resultado);
```
