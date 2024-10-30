use crate::utilitis::hardware::{leds::{self, Leds}, placa_arm::PlacaARM};
use super::operaciones::Operacion;

// Estructura `InstruccionBinaria` con los métodos completos
/*
Bits 31-28 (Condición): Determinan la condición bajo la cual se ejecuta la instrucción (por ejemplo, EQ, NE, GT, etc.).
Bit 27 (I): Indica si el Operand2 es inmediato (1) o un registro (0).
Bits 26-25 (OPCODE): Indican el tipo de instrucción que se va a ejecutar (por ejemplo, operaciones aritméticas).
Bit 24 (S): Indica si se debe actualizar el registro de estado (1) o no (0).
Bits 23-20 (Rn): Registro base (primer operando).
Bits 19-16 (Rd): Registro de destino (donde se almacenará el resultado).
Bits 15-0 (Operand2): Puede ser un valor inmediato o un desplazamiento, dependiendo de si I es 1 o 0. 
*/
pub struct InstruccionBinaria {}

impl InstruccionBinaria {
    pub fn new() -> Self {
        InstruccionBinaria {}
    }

    pub fn llamado(&self, array_bits: &[i32], placa: &mut PlacaARM) {
        if array_bits.len() < 32 {
            println!("Error: longitud insuficiente de array_bits");
            return;
        }

        let rd = self.rd(array_bits);
        let rn = self.rn(array_bits);
        let operand2 = self.operand2(array_bits);
        let mut instruccion_binaria = Operacion::new();

        self.cond(array_bits, rd, rn, operand2, &mut instruccion_binaria, placa);
    }

    fn cond(&self, array_bits: &[i32], rd: i32, rn: i32, operand2: i32, instruccion_binaria: &mut Operacion, placa: &mut PlacaARM) {
        let offset = self.obtener_offset(array_bits);

        match &array_bits[0..4] {
            [0, 0, 0, 0] => instruccion_binaria.beq(placa, offset),
            [0, 0, 0, 1] => instruccion_binaria.bne(placa, offset),
            [0, 0, 1, 0] => instruccion_binaria.bgt(placa, offset),
            [0, 0, 1, 1] => instruccion_binaria.blt(placa, offset),
            [0, 1, 0, 0] => instruccion_binaria.bge(placa, offset),
            [0, 1, 0, 1] => instruccion_binaria.ble(placa, offset),
            [0, 1, 1, 0] => instruccion_binaria.bhi(placa, offset),
            [0, 1, 1, 1] => instruccion_binaria.bls(placa, offset),
        //  [1, 0, 0, 0] => instruccion_binaria.bvc(placa, offset),
        //  [1, 0, 0, 1] => instruccion_binaria.bvs(placa, offset),
        //  [1, 0, 1, 0] => instruccion_binaria.bcc(placa, offset),
        //  [1, 0, 1, 1] => instruccion_binaria.bcs(placa, offset),
        //  [1, 1, 0, 0] => instruccion_binaria.bpl(placa, offset),
            [1, 1, 0, 1] => instruccion_binaria.bmi(placa, offset),
            [1, 1, 1, 0] => self.bit_tipo_de_instruccion(array_bits, rd, rn, operand2, instruccion_binaria, placa),
            [1, 1, 1, 1] => println!("Reservado"),
            _ => println!("Condición no válida."),
        }
    }

    fn bit_tipo_de_instruccion(&self, array_bits: &[i32], rd: i32, rn: i32, operand2: i32, instruccion_binaria: &mut Operacion, placa: &mut PlacaARM) {
        match array_bits[4..7] { // Acceder a los bits 27-25
            [0, 0, 0] => {
                // Procesamiento de datos (Data Processing)
                self.opcodes(array_bits, rd, rn, operand2, instruccion_binaria, placa, false);
            }
            [0, 0, 1] => {
                // Procesamiento de datos con inmediato (Immediate Data Processing)
                self.opcodes(array_bits, rd, rn, operand2, instruccion_binaria, placa, true);
            }
            [0, 1, 0] => {
                // Load/Store inmediato (Immediate Load/Store)
                match array_bits[10..12] { // Acceder a los bits 22-21
                    [0, 0] => instruccion_binaria.ldr(rd.try_into().unwrap(), rn.try_into().unwrap(), operand2, placa), // LDR (Load Register)
                    [0, 1] => instruccion_binaria.str(rd.try_into().unwrap(), rn.try_into().unwrap(), operand2, placa), // STR (Store Register)
                    _ => println!("Tipo de Load/Store no reconocido"),
                }
            }
            [0, 1, 1] => {
                // Load/Store registro (Register Load/Store)
                match array_bits[10..12] { // Acceder a los bits 22-21
                    [0, 0] => instruccion_binaria.ldr(rd.try_into().unwrap(), rn.try_into().unwrap(), operand2, placa), // LDR (Load Register)
                    [0, 1] => instruccion_binaria.str(rd.try_into().unwrap(), rn.try_into().unwrap(), operand2, placa), // STR (Store Register)
                    _ => println!("Tipo de Load/Store no reconocido"),
                }
            }
            [1, 0, 0] => {
                // Load/Store múltiple (Multiple Load/Store)
                println!("Load/Store múltiple no implementado");
            }
            [1, 0, 1] => {
                // Branch and Link (BL)
                instruccion_binaria.bl(placa, operand2); // Realizar el Branch and Link
            }
            [1, 1, 0] => {
                // Coprocesador Load/Store (Coprocessor Load/Store)
                println!("Coprocesador Load/Store no implementado");
            }
            [1, 1, 1] => {
                // Interrupción de software (Software Interrupt - SWI)
                instruccion_binaria.swi(placa); // Ejecutar la interrupción de software
            }
            _ => println!("No coincide con ninguna secuencia conocida"),
        }
    }


    fn opcodes(&self, array_bits: &[i32], rd: i32, rn: i32, operand2: i32, instruccion_binaria: &mut Operacion, placa: &mut PlacaARM, inmediato: bool) {
        match &array_bits[7..11] {
            [0, 0, 0, 0] => instruccion_binaria.and(placa, rd, rn, operand2, inmediato, self.bit_s(array_bits)),
            [0, 0, 0, 1] => instruccion_binaria.eor(placa, rd, rn, operand2, inmediato, self.bit_s(array_bits)),
            [0, 0, 1, 0] => instruccion_binaria.sub(placa, rd, rn, operand2, inmediato, self.bit_s(array_bits)),
            [0, 1, 0, 0] => instruccion_binaria.add(placa, rd, rn, operand2, inmediato, self.bit_s(array_bits)),
            [1, 0, 0, 0] => instruccion_binaria.tst(placa, rd, rn, inmediato),
            [1, 0, 1, 0] => instruccion_binaria.cmp(placa, rd, rn, inmediato),
            [1, 1, 0, 1] => instruccion_binaria.mov(placa, rd, rn, inmediato),
            [1, 1, 1, 1] => instruccion_binaria.mvn(placa, rd, operand2, inmediato),

            // Manejo de instrucciones de salto
            [1, 1, 0, 0] => {
                // Instrucción de salto B
                let offset = self.obtener_offset(array_bits);
                instruccion_binaria.b(placa, offset); // Llama a la función de salto
            }
            [1, 1, 1, 0] => {
                // Instrucción de salto BL
                let offset = self.obtener_offset(array_bits);
                instruccion_binaria.bl(placa, offset); // Llama a la función de salto y enlace
            }

            _ => println!("Opcode no reconocido"),
        }
    }


    fn bit_s(&self, array_bits: &[i32]) -> bool {
        array_bits[20] == 1
    }

    /*fn obtener_offset(&self,array_bits: &[i32]) -> i32 {
        // Obtener los últimos 24 bits de `array_bits`, que contienen el offset
        let offset_24_bits = self.bits_a_decimal(&array_bits[8..32]);

        // Convertir a complemento a dos si el valor es negativo
        let offset_signed = if offset_24_bits & 0x800000 != 0 {
            offset_24_bits | !0xFFFFFF // Extiende el signo
        } else {
            offset_24_bits
        };

        // Desplazar dos bits a la izquierda para obtener el offset en bytes
        let offset_bytes = offset_signed << 2;

        // Retorna el offset ajustado (por ejemplo, `0x8`)
        offset_bytes
    }*/
    
    fn obtener_offset(&self, array_bits: &[i32]) -> i32 {
        let offset = self.bits_a_decimal(&array_bits[7..32]); // Obtener los últimos 24 bits de `array_bits`

        // Convertir a complemento a dos si el valor es negativo
        let offset_signed = if offset & 0x800000 != 0 {
            offset | !0xFFFFFF // Extiende el signo
        } else {
            offset
        };

        // Desplazar dos bits a la izquierda para obtener el offset en bytes
        offset_signed << 2 // Desplazamiento a bytes
    }

    fn bits_a_decimal(&self, array_bits: &[i32]) -> i32 {
        array_bits.iter().rev().enumerate().map(|(i, &bit)| bit * (1 << i)).sum()
    }

    fn rd(&self, array_bits: &[i32]) -> i32 {
        self.bits_a_decimal(&array_bits[16..20])
    }

    fn rn(&self, array_bits: &[i32]) -> i32 {
        self.bits_a_decimal(&array_bits[12..16])
    }

    fn operand2(&self, array_bits: &[i32]) -> i32 {
        // Si operand2 puede ser un registro, usa los bits 0-3 para un registro
        // o los bits 0-11 para un valor inmediato
        if array_bits[6] == 1 { // bit 25 indica que es inmediato
            // Leer bits 0 a 11 para el valor inmediato
            self.bits_a_decimal(&array_bits[20..32]) // Esto es para el caso inmediato
        } else {
            // Leer bits 16 a 19 para un registro (R0 a R15)
            self.bits_a_decimal(&array_bits[28..32]) // Esto es para el caso registro
        }
    }
}