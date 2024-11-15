use crate::utilitis::hardware::{leds::{self, Leds}, placa_arm::PlacaARM};
use super::operaciones::Operacion;
pub struct InstruccionBinaria {}

impl InstruccionBinaria {
    pub fn new() -> Self {
        InstruccionBinaria {}
    }

    pub fn llamado(&self, array_bits: &[i32], placa: &mut PlacaARM) {
        if array_bits.len() != 32 {
            println!("Error: Se requieren 32 bits exactamente. Recibidos: {}", array_bits.len());
            return;
        }

        // Extraer los campos necesarios del vector de bits
        let cond = &array_bits[0..4];  // Condition field (31-28)
        let op_type = &array_bits[4..6];  // bits [27:26]
        let is_immediate = array_bits[6];  // bit 25
        let s_bit = array_bits[24];        // bit 7
        let opcode = &array_bits[7..11];   // bits [24:21]
        
        // Extraer registros y operando
        let rn = self.bits_to_decimal(&array_bits[12..16]);  // bits [19:16]
        let rd = self.bits_to_decimal(&array_bits[16..20]);  // bits [15:12]
        let operand2 = if op_type == [0, 1] {
            self.bits_to_decimal(&array_bits[20..32])
        } else {
            if is_immediate == 1 {
                self.bits_to_decimal(&array_bits[20..32])
            } else {
                self.bits_to_decimal(&array_bits[28..32])
            }
        };

        let mut operacion = Operacion::new();

        // Proceso de decodificación basado en los tipos de operación
        match op_type {
            // Data Processing (00)
            [0, 0] => {
                self.execute_data_processing(
                    opcode, 
                    rd, 
                    rn, 
                    operand2, 
                    is_immediate == 1, 
                    s_bit == 1, 
                    &mut operacion, 
                    placa
                );
            },
            // Memory Access (01)
            [0, 1] => {
                let is_load = array_bits[11] == 1;
                if is_load {
                    operacion.ldr(placa, rd, rn, operand2, is_immediate == 1, s_bit == 1);
                } else {
                    operacion.str(placa, rd, rn, operand2, is_immediate == 1, s_bit == 1);
                }
            },
            // Branch (10)
            [1, 0] => {
                let offset = self.calculate_branch_offset(array_bits);
                println!("offset: {offset}");
                
                // Verificar la condición del branch
                match cond {
                    // 1110 - AL (Always, usado para B normal)
                    [1, 1, 1, 0] => {
                        operacion.b(placa, offset);
                    },
                    // 0000 - EQ (Equal, Z set)
                    [0, 0, 0, 0] => {
                        match placa.get_flag(0) {  // Se pasa el índice adecuado de la bandera
                            Some(flag) => {
                                if flag {
                                    println!("BEQ tomado (Z=1)");
                                    operacion.b(placa, offset);
                                } else {
                                    println!("BEQ no tomado (Z=0)");
                                    // No realizar el salto, continuar con la siguiente instrucción
                                }
                            }
                            None => {
                                println!("Error: No se pudo obtener la bandera Z");
                                // Manejo de error si el índice no es válido o el valor no está disponible
                            }
                        }
                    },
                    // 0001 - NE (Not Equal, Z clear)
                    [0, 0, 0, 1] => {
                        match placa.get_flag(0) {  // Se pasa el índice adecuado de la bandera
                            Some(flag) => {
                                if !flag {
                                    println!("BNE tomado (Z=0)");
                                    operacion.b(placa, offset);
                                } else {
                                    println!("BNE no tomado (Z=1)");
                                    // No realizar el salto, continuar con la siguiente instrucción
                                }
                            }
                            None => {
                                println!("Error: No se pudo obtener la bandera Z");
                                // Manejo de error si el índice no es válido o el valor no está disponible
                            }
                        }
                    },
                    _ => println!("Condición de branch no implementada: {:?}", cond),
                }
            },
            // SWI (11)
            [1, 1] => {
                operacion.swi(placa);
            },
            _ => println!("Tipo de operación no válido"),
        }
    }

    fn execute_data_processing(
        &self,
        opcode: &[i32],
        rd: i32,
        rn: i32,
        operand2: i32,
        is_immediate: bool,
        s_bit: bool,
        operacion: &mut Operacion,
        placa: &mut PlacaARM
    ) {
        match opcode {
            [0, 0, 0, 0] => operacion.and(placa, rd, rn, operand2, is_immediate, s_bit),
            [0, 0, 0, 1] => operacion.eor(placa, rd, rn, operand2, is_immediate, s_bit),
            [0, 0, 1, 0] => operacion.sub(placa, rd, rn, operand2, is_immediate, s_bit),
            [0, 1, 0, 0] => operacion.add(placa, rd, rn, operand2, is_immediate, s_bit),
            _ => println!("Opcode no reconocido: {:?}", opcode),
        }
    }

    fn bits_to_decimal(&self, bits: &[i32]) -> i32 {
        bits.iter()
            .enumerate()
            .fold(0, |acc, (i, &bit)| {
                acc + (bit * (1 << (bits.len() - 1 - i)))
            })
    }

   fn calculate_branch_offset(&self, array_bits: &[i32]) -> i32 {
        // Extraer los 24 bits de offset
        let offset_bits = &array_bits[8..32];
        let mut offset = self.bits_to_decimal(offset_bits);

        // Realizar extensión de signo si es necesario (bit 23 es 1)
        if offset_bits[0] == 1 {
            offset |= -0x1000000; // Extender el signo para un valor de 24 bits
        }

        // Multiplicar el offset por 4 (como espera la función b())
        offset *= 4;

        // La función b() ya maneja el PC+8, así que aquí solo devolvemos el offset directamente
        offset
    }
}
