use crate::utilitis::hardware::placa_arm::{self, PlacaARM};
use super::operaciones::Operacion;

pub struct BinarioToOperacion {}

impl BinarioToOperacion {
    pub fn new() -> Self {
        BinarioToOperacion {}
    }

    pub fn cond(&self, binarioV: &[i32], opera: &mut Operacion, placa: &mut PlacaARM) {
        let offset = self.obtener_offset(binarioV);

        match binarioV[0..4] {
            [0, 0, 0, 0] => opera.beq(placa, offset),
            [0, 0, 0, 1] => opera.bne(placa, offset),
            [0, 1, 1, 1] => opera.b(placa, offset),
            _ => println!("Condición no válida."),
        }
    }

    pub fn obtener_offset(&self, binarioV: &[i32]) -> i32 {
        let offset_bits = &binarioV[4..28];
        let offset = self.bits_a_decimal(offset_bits);
        offset >> 2
    }

    pub fn tipo_de_instruccion(&self, binarioV: &[i32]) -> i32 {
        if binarioV.len() < 6 {
            return 0;
        }

        match (binarioV[4], binarioV[5]) {
            (0, 0) => 1, // Instrucción de Tipo R
            (0, 1) => 2, // Instrucción de Carga/Almacenamiento
            (1, _) => 3, // Instrucción de Salto
            _ => 0,
        }
    }

    pub fn opcode(&self, binarioV: &[i32], opera: &mut Operacion, placa: &mut PlacaARM) {
        let rd = self.rd(binarioV);
        let rn = self.rn(binarioV);
        let operand2 = self.operand2(binarioV);
        let tipoDeOperacion = self.tipo_de_instruccion(binarioV);

        match tipoDeOperacion {
            1 => self.handle_tipo_r(binarioV, rd, rn, operand2, opera, placa),
            2 => self.handle_carga_almacenamiento(binarioV, rd, operand2, opera, placa),
            3 => self.cond(binarioV, opera, placa),
            _ => println!("Tipo de operación no válido."),
        }
    }

    fn handle_tipo_r(&self, binarioV: &[i32], rd: i32, rn: i32, operand2: i32, opera: &mut Operacion, placa: &mut PlacaARM) {
        match binarioV[7..11] {
            [0, 1, 0, 0] => opera.add(placa, rd as usize, rn as usize, operand2),
            [0, 0, 1, 0] => opera.sub(placa, rd as usize, rn as usize, operand2),
            [0, 0, 0, 0] => opera.and(placa, rd as usize, rn as usize, operand2),
            [1, 1, 0, 0] => opera.orr(placa, rd as usize, rn as usize, operand2),
            _ => println!("Operación de Tipo R no válida."),
        }
    }

    fn handle_carga_almacenamiento(&self, binarioV: &[i32], rd: i32, operand2: i32, opera: &mut Operacion, placa: &mut PlacaARM) {
        match binarioV[7..11] {
            [0, 1, 0, 1] => opera.ldr(placa, rd as usize, operand2),
            [0, 1, 1, 0] => opera.str(placa, operand2, rd as i32),
            _ => println!("Operación de carga/almacenamiento no válida."),
        }
    }

    pub fn bit_i(&self, binarioV: &[i32]) -> bool {
        binarioV[6] == 0 // El operando es un registro si es 0
    }

    pub fn bit_s(&self, binarioV: &[i32]) {
        if binarioV[11] == 1 {
            // Actualiza las banderas de estado en el registro CPSR.
        }
    }

    pub fn rn(&self, binarioV: &[i32]) -> i32 {
        if binarioV.len() >= 16 {
            self.registro(binarioV[10], binarioV[11], binarioV[12], binarioV[13])
        } else {
            0
        }
    }

    pub fn rd(&self, binarioV: &[i32]) -> i32 {
        if binarioV.len() >= 16 {
            self.registro(binarioV[14], binarioV[15], binarioV[16], binarioV[17])
        } else {
            0
        }
    }

    pub fn operand2(&self, binarioV: &[i32]) -> i32 {
        if binarioV.len() >= 20 {
            let operand2_bits = &binarioV[18..30];
            self.bits_a_decimal(operand2_bits)
        } else {
            0
        }
    }

    pub fn bits_a_decimal(&self, bits: &[i32]) -> i32 {
        bits.iter().enumerate().fold(0, |acc, (i, &bit)| acc | (bit << (11 - i)))
    }

    pub fn registro(&self, a: i32, b: i32, c: i32, d: i32) -> i32 {
        (a << 3) | (b << 2) | (c << 1) | d
    }
}
