use crate::utilitis::{archivos::archivo::Archivo, hardware::placa_arm::PlacaARM, traduccion::operaciones::Operacion};

use super::instrucciones_binarias;


pub struct Traductor {}

impl Traductor {
    pub fn new() -> Self {
        Traductor {}
    }

    pub fn convertir(&self) {
        let archivo = Archivo::new("./src/utilitis/archivos/imem_io.dat");
        let mut placa = PlacaARM::new();
        let instrucciones_bina = instrucciones_binarias::InstruccionBinaria::new();

        let instrucciones_hex = archivo
            .lectura_instrucciones()
            .expect("Error al leer instrucciones");

        for instruccion_hex in instrucciones_hex {
            let hex_string: String = instruccion_hex.iter().collect();
            let binario_str = self.hex_string_to_binary(&hex_string);
            let bits = self.separar_binario_en_vector(&binario_str);

            instrucciones_bina.llamado(&bits, &mut placa);
            if let Some(pc) = placa.get_register(15) {
                if pc >= instrucciones_hex.len() as u32 {
                    break;
                }
            }
        }
    }

    // Convierte un carácter hexadecimal a binario (4 bits)
    fn hex_char_to_binary(&self, c: char) -> String {
        let num = c.to_digit(16).expect("Error al convertir carácter hexadecimal a decimal");
        format!("{:04b}", num)
    }

    // Convierte una cadena hexadecimal completa en una cadena binaria
    fn hex_string_to_binary(&self, hex: &str) -> String {
        let mut binary_representation = String::new();
        for c in hex.chars() {
            let binary = self.hex_char_to_binary(c);
            binary_representation.push_str(&binary);
        }
        binary_representation
    }

    // Separar la cadena binaria en bits y almacenarlos en un vector
    fn separar_binario_en_vector(&self, cadena: &str) -> Vec<u32> {
        cadena.chars()
            .filter_map(|c| c.to_digit(2))
            .collect()
    }

    // Método para extraer componentes
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

        // Retornar los valores extraídos
        (cond, i, opcode, rn, rd, operand2)
    }
} // Esta llave de cierre cierra la implementación de Traductor
