use crate::utilitis::hardware::{leds::{self, Leds}, placa_arm::PlacaARM};
use super::operaciones::Operacion;

// Estructura `InstruccionBinaria` con los métodos completos
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
            _ => println!("Opcode no reconocido"),
        }
    }

    fn bit_s(&self, array_bits: &[i32]) -> bool {
        array_bits[20] == 1
    }

    fn obtener_offset(&self, array_bits: &[i32]) -> i32 {
        self.bits_a_decimal(&array_bits[0..12])
    }

    fn bits_a_decimal(&self, array_bits: &[i32]) -> i32 {
        array_bits.iter().rev().enumerate().map(|(i, &bit)| bit * (1 << i)).sum()
    }

    fn rd(&self, array_bits: &[i32]) -> i32 {
        self.bits_a_decimal(&array_bits[12..16])
    }

    fn rn(&self, array_bits: &[i32]) -> i32 {
        self.bits_a_decimal(&array_bits[16..20])
    }

    fn operand2(&self, array_bits: &[i32]) -> i32 {
        self.bits_a_decimal(&array_bits[0..12])
    }
}
