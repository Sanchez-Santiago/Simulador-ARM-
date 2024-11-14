use crate::utilitis::{archivos::archivo::Archivo, hardware::placa_arm::PlacaARM};
use super::instrucciones_binarias;
use std::convert::TryInto;

pub struct Traductor {}

impl Traductor {
    pub fn new() -> Self {
        Traductor {}
    }

    pub fn convertir(&self, placa: &mut PlacaARM) {
        let archivo = Archivo::new("src/utilitis/archivos/imem_io.dat");
        let instrucciones_bina = instrucciones_binarias::InstruccionBinaria::new();
        
        let instrucciones_h = match archivo.lectura_instrucciones() {
            Ok(instr) => instr,
            Err(e) => {
                eprintln!("Error al leer instrucciones: {:?}", e);
                return;
            }
        };

        let mut pc = 0; // Inicializa el PC en 0

        while (pc / 4) < instrucciones_h.len() {
            if pc >= instrucciones_h.len() * 4 {
                break; // Sale del bucle si el PC es mayor que la última posición válida
            }
            
            // Convierte cada instrucción de Vec<char> a String
            let hex_string: String = instrucciones_h[pc / 4].iter().collect();
            // Convierte la cadena hexadecimal a una representación binaria
            let binario_str = self.hex_string_to_binary(&hex_string);
            // Separa la cadena binaria en un vector de i32
            let bits = self.separar_binario_en_vector(&binario_str);

            // Llama a llmado con el vector de bits
            instrucciones_bina.llamado(&bits.iter().map(|&b| b as i32).collect::<Vec<i32>>(), placa);
            
            // Actualiza el PC y almacena en R15
            pc = placa.get_register(15).unwrap_or(0) as usize + 4; // Aumenta el PC por cada instrucción ejecutada
            placa.set_register(15, pc as i32); // Guarda el nuevo valor del PC en R15
        }
    }

    fn hex_string_to_binary(&self, hex: &str) -> String {
        hex.chars()
            .map(|c| self.hex_char_to_binary(c))
            .collect()
    }

    fn hex_char_to_binary(&self, c: char) -> String {
        let num = c.to_digit(16).expect("Error al convertir carácter hexadecimal a decimal");
        format!("{:04b}", num)
    }

    fn separar_binario_en_vector(&self, cadena: &str) -> Vec<i32> {
        cadena.chars()
            .filter_map(|c| {
                if c == '0' || c == '1' {
                    Some(c.to_digit(2).unwrap() as i32)
                } else {
                    None
                }
            })
            .collect()
    }

    fn extraer_componentes(&self, binario: &[u32]) -> (u32, u32, u32, u32, u32, u32) {
        if binario.len() < 32 {
            panic!("El vector binario debe contener al menos 32 bits.");
        }

        // Extraer los componentes según la posición de bits
        let cond = (binario[0] << 3) | (binario[1] << 2) | (binario[2] << 1) | binario[3]; // bits 31-28
        let i = binario[25]; // bit 25
        let opcode = (binario[21] << 3) | (binario[22] << 2) | (binario[23] << 1) | binario[24]; // bits 24-21
        let rn = (binario[16] << 3) | (binario[17] << 2) | (binario[18] << 1) | binario[19]; // bits 19-16
        let rd = (binario[12] << 3) | (binario[13] << 2) | (binario[14] << 1) | binario[15]; // bits 15-12
        let operand2 = (binario[0] << 11) | (binario[1] << 10) | (binario[2] << 9) | (binario[3] << 8) |
                        (binario[4] << 7) | (binario[5] << 6) | (binario[6] << 5) | (binario[7] << 4) |
                        (binario[8] << 3) | (binario[9] << 2) | (binario[10] << 1) | binario[11]; // bits 11-0

        // Retornar los valores extraídos, conviertiendo a u32
        (
            cond.try_into().unwrap(),
            i.try_into().unwrap(),
            opcode.try_into().unwrap(),
            rn.try_into().unwrap(),
            rd.try_into().unwrap(),
            operand2.try_into().unwrap()
        )
    }
}
