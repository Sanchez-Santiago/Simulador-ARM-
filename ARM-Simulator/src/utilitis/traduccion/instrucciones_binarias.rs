use crate::utilitis::hardware::{leds::{self, Leds}, placa_arm::PlacaARM};
use super::operaciones::Operacion;
/*
Bits	    Campo	    Descripción
31 - 28	    Cond	    Condición de ejecución: especifica cuándo ejecutar la instrucción (por ejemplo, si es igual o no igual).
27	        Tipo	    Primer bit que ayuda a definir el tipo de instrucción.
26	        Tipo	    Segundo bit que, en combinación con el anterior, define si la instrucción es de procesamiento de datos, carga/almacenamiento, transferencia de bloques, salto, etc.
25	        I	        Bit de inmediato: indica si el Operand2 es un valor inmediato o un registro.
24 - 21	    Opcode	    Código de operación: define el tipo de operación (como ADD, SUB, MOV, etc.).
20	        S	        Flag de seteo de condición: indica si la operación debe actualizar las banderas del registro CPSR.
19 - 16	    Rn	        Registro fuente o base: uno de los registros de entrada.
15 - 12	    Rd	        Registro de destino: donde se almacena el resultado de la operación.
11 - 0	    Operand2	Segundo operando, que puede ser un valor inmediato, un registro o un valor desplazado dependiendo del bit I.
*/

pub struct InstruccionBinaria {}

impl InstruccionBinaria {
    pub fn new() -> Self {
        InstruccionBinaria {}
    }

    pub fn llamado(&self, array_bits: &[i32], placa: &mut PlacaARM) {
        // Verifica que el array tenga al menos 32 bits
        if array_bits.len() < 32 {
            println!("Error: longitud insuficiente de array_bits");
            return;
        }

        let rd = self.rd(array_bits); // Obtener el registro de destino
        let rn = self.rn(array_bits); // Obtener el registro fuente/base
        let operand2 = self.operand2(array_bits); // Obtener el segundo operando
        let mut instruccion_binaria = Operacion::new(); // Crear una nueva instancia de Operacion

        // Determinar el tipo de instrucción y ejecutarla
        self.cond(array_bits, rd, rn, operand2, &mut instruccion_binaria, placa);
    }

    // Cond del bits 31 a 28
    fn cond(&self, array_bits: &[i32], rd: i32, rn: i32, operand2: i32, instruccion_binaria: &mut Operacion, placa: &mut PlacaARM) {
        // Verifica que el array tenga al menos 4 bits para la condición
        if array_bits.len() < 4 {
            println!("Error: longitud insuficiente para condición");
            return;
        }

        let offset = self.obtener_offset(array_bits); // Obtener el offset para las instrucciones de salto

        // Revisar los primeros cuatro bits para determinar la condición
        match array_bits[0..4] {
            [0, 0, 0, 0] => instruccion_binaria.beq(placa, offset), // BEQ: Branch if Equal
            [0, 0, 0, 1] => instruccion_binaria.bne(placa, offset), // BNE: Branch if Not Equal
            [0, 0, 1, 0] => instruccion_binaria.bgt(placa, offset), // BGT: Branch if Greater Than
            [0, 0, 1, 1] => instruccion_binaria.blt(placa, offset), // BLT: Branch if Less Than
            [0, 1, 0, 0] => instruccion_binaria.bge(placa, offset), // BGE: Branch if Greater Than or Equal
            [0, 1, 0, 1] => instruccion_binaria.ble(placa, offset), // BLE: Branch if Less Than or Equal
            [0, 1, 1, 0] => instruccion_binaria.bhi(placa, offset), // BHI: Branch if Higher
            [0, 1, 1, 1] => instruccion_binaria.bls(placa, offset), // BLS: Branch if Lower or Same
            [1, 0, 0, 0] => instruccion_binaria.bvc(placa, offset), // BVC: Branch if Overflow Clear
            [1, 0, 0, 1] => instruccion_binaria.bvs(placa, offset), // BVS: Branch if Overflow Set
            [1, 0, 1, 0] => instruccion_binaria.bcc(placa, offset), // BCC: Branch if Carry Clear
            [1, 0, 1, 1] => instruccion_binaria.bcs(placa, offset), // BCS: Branch if Carry Set
            [1, 1, 0, 0] => instruccion_binaria.bpl(placa, offset), // BPL: Branch if Plus
            [1, 1, 0, 1] => instruccion_binaria.bmi(placa, offset), // BMI: Branch if Minus
            [1, 1, 1, 0] => self.bit_tipo_de_instruccion(array_bits, rd, rn, operand2, instruccion_binaria, placa),   // B: Branch (unconditional)
            [1, 1, 1, 1] => println!("Reservado"),          // Reservado
            _ => println!("Condición no válida."),          // Condición no válida
        }
    }

    // Tipos de Instrucciones ARM bits 27-26-25
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
                    [0, 0] => instruccion_binaria.ldr(rd, operand2, placa), // LDR (Load Register)
                    [0, 1] => instruccion_binaria.str(rd, operand2, placa), // STR (Store Register)
                    _ => println!("Tipo de Load/Store no reconocido"),
                }
            }
            [0, 1, 1] => {
                // Load/Store registro (Register Load/Store)
                match array_bits[10..12] { // Acceder a los bits 22-21
                    [0, 0] => instruccion_binaria.ldr(rd, rn, operand2, placa), // LDR (Load Register)
                    [0, 1] => instruccion_binaria.str(rd, rn, operand2, placa), // STR (Store Register)
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
        // Acceder a los bits 24-21 para determinar la operación
        match array_bits[7..11] {
            [0, 0, 0, 0] => instruccion_binaria.and(placa, rd, rn, operand2, inmediato, self.bit_s(array_bits)), // AND
            [0, 0, 0, 1] => instruccion_binaria.eor(placa, rd, rn, operand2, inmediato, self.bit_s(array_bits)), // EOR
            [0, 0, 1, 0] => instruccion_binaria.sub(placa, rd, rn, operand2, inmediato, self.bit_s(array_bits)), // SUB
            [0, 0, 1, 1] => instruccion_binaria.rsb(placa, rd, rn, operand2), // RSB
            [0, 1, 0, 0] => instruccion_binaria.add(placa, rd, rn, operand2, inmediato, self.bit_s(array_bits)), // ADD
            [0, 1, 0, 1] => instruccion_binaria.adc(placa, rd, rn, operand2), // ADC
            [0, 1, 1, 0] => instruccion_binaria.sbc(placa, rd, rn, operand2), // SBC
            [0, 1, 1, 1] => instruccion_binaria.rsc(placa, rd, rn, operand2), // RSC
            [1, 0, 0, 0] => instruccion_binaria.tst(placa, rd, rn, inmediato), // TST
            [1, 0, 0, 1] => instruccion_binaria.teq(placa, rd, rn, inmediato), // TEQ
            [1, 0, 1, 0] => instruccion_binaria.cmp(placa, rd, rn, inmediato), // CMP
            [1, 0, 1, 1] => instruccion_binaria.cmn(placa, rd, rn), // CMN
            [1, 1, 0, 0] => instruccion_binaria.orr(placa, rd, rn, operand2, inmediato, self.bit_s(array_bits)), // ORR
            [1, 1, 0, 1] => instruccion_binaria.mov(placa, rd, rn,  inmediato), // MOV
            [1, 1, 1, 0] => instruccion_binaria.bic(placa, rd, rn, operand2, inmediato, self.bit_s(array_bits)), // BIC
            [1, 1, 1, 1] => instruccion_binaria.mvn(placa, rd, operand2, inmediato), // MVN
            _ => println!("Opcode no reconocido"),
        }
    }


    fn bit_s(&self, array_bits: &[i32]) -> bool {
        // Retorna el valor del bit S (bit 20)
        array_bits[20] == 1
    }

    fn obtener_offset(&self, array_bits: &[i32]) -> i32 {
        // Obtener el offset en decimal desde el arreglo de bits
        self.bits_a_decimal(&array_bits[0..12]) // Acceder a los bits 11-0
    }

    fn bits_a_decimal(&self, array_bits: &[i32]) -> i32 {
        // Convierte los bits en decimal
        array_bits.iter().rev().enumerate().map(|(i, &bit)| bit * (1 << i)).sum()
    }

    fn rd(&self, array_bits: &[i32]) -> i32 {
        // Extraer el registro de destino (bits 15-12)
        self.bits_a_decimal(&array_bits[12..16])
    }

    fn rn(&self, array_bits: &[i32]) -> i32 {
        // Extraer el registro base (bits 19-16)
        self.bits_a_decimal(&array_bits[16..20])
    }

    fn operand2(&self, array_bits: &[i32]) -> i32 {
        // Extraer el segundo operando (bits 11-0)
        self.bits_a_decimal(&array_bits[0..12])
    }
}
