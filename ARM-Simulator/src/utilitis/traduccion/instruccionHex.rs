use crate::utilitis::archivos::archivo::Archivo;
use crate::utilitis::hardware::placa_arm::PlacaARM;
use crate::utilitis::hardware::leds;
use super::operaciones::Operacion;
use std::io;

/*
Codificación en Hexadecimal

La codificación en hexadecimal depende de los operandos y de su modo de operación. Para el ejemplo anterior, la instrucción SUBS R2, R2, R1 sería representada en hexadecimal como sigue:

    Identificación de la instrucción:
        El código de operación para SUBS es 001110 (en binario).
        Esta instrucción pertenece al formato de tipo R.

    Formato R:
    El formato R en ARM es el siguiente:

    cond 00 I opcode S Rn Rd Rm

    Donde:
        cond: condición (4 bits)
        00: 2 bits (siempre 00 para instrucciones de tipo R)
        I: 1 bit (0 para operaciones de registro)
        opcode: 6 bits (código de operación)
        S: 1 bit (1 para SUBS)
        Rn: registro de origen (4 bits)
        Rd: registro de destino (4 bits)
        Rm: segundo operando (4 bits)

Ejemplo de codificación:

    Para SUBS R2, R2, R1:
        cond: 1110 (condición siempre verdadera)
        I: 0
        opcode para SUB: 001000
        S: 1 (indica que se actualizan las banderas)
        Rn: R2 es 0010
        Rd: R2 es 0010
        Rm: R1 es 0001

Combinando todo, tendríamos:

1110 00 0 001000 1 0010 0010 0001


4. **Conversión a Hexadecimal:**
   - `111000000010001100100001` en binario se convierte a hexadecimal:
     - Binario: `111000 00 00010 00100 0001`
     - Hexadecimal: `E2020001`

Por lo tanto, **la instrucción `SUBS R2, R2, R1` se codifica como `E2020001` en hexadecimal**.

### Resumen

Para diferentes operandos y registros, la codificación variará. Pero el procedimiento será similar: identificar la operación y los registros, y luego convertir a binario y finalmente a hexadecimal.
*/
pub struct InstruccionesHex {}

impl InstruccionesHex {
    pub fn new() -> Self {
        InstruccionesHex {}
    }

    // Hacemos que el método "convertir" reciba &self
    pub fn convertir(&self) {
        let operaciones = Operacion::new();
        let archivo = Archivo::new("./utilitis/archivos/imem_io.dat");
        let instruccionesH = archivo
            .lectura_instrucciones()
            .expect("Error al leer instrucciones"); // Obtener las instrucciones
        let _led = leds::Leds::new(); // `_led` se inicializa pero no se utiliza. Considera usarlo o eliminarlo.

        for linea in instruccionesH {
            for charL in linea {
                let binary_representation = self.hex_char_to_binary(charL); // Ahora puedes usar 'self' aquí
                println!("Carácter: {}, Binario: {}", charL, binary_representation);
            }
        }
    }

    fn hex_char_to_binary(&self, c: char) -> String {
        // Convierte el carácter hexadecimal a un número entero
        let num = c.to_digit(16).expect("Error al convertir carácter hexadecimal a decimal");
        // Convierte el número entero a binario y asegura que tenga 4 bits
        format!("{:04b}", num)
    }

    fn hex_string_to_binary(&self, hex: &str) -> String {
        let mut binary_representation = String::new();

        for c in hex.chars() {
            // Convierte cada carácter hexadecimal a binario
            let binary = self.hex_char_to_binary(c);
            binary_representation.push_str(&binary); // Agrega la representación binaria
        }

        binary_representation
    }
}