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

        // Inicializamos el PC como i32
        let mut pc: i32 = 0;
        placa.set_register(15, pc);

        while (pc / 4) < instrucciones_h.len().try_into().unwrap() {
            let current_pc = pc as usize;

            // Convertir la instrucción actual
            let hex_string: String = instrucciones_h[current_pc / 4].iter().collect();
            match self.hex_string_to_binary(&hex_string) {
                Ok(binario_str) => {
                    let bits = self.separar_binario_en_vector(&binario_str);

                    // Ejecutar la instrucción
                    instrucciones_bina.llamado(
                        &bits.iter().map(|&b| b as i32).collect::<Vec<i32>>(),
                        placa,
                    );

                    // Obtener el nuevo PC después de la ejecución
                    pc = placa.get_register(15).unwrap_or(pc);

                    // Verificación de seguridad para evitar bucles infinitos
                    if pc >= (instrucciones_h.len() as i32 * 4) {
                        break;
                    }

                    // Incrementamos el PC antes de ejecutar la siguiente instrucción
                    pc += 4;
                    placa.set_register(15, pc); // PC+8 desde la perspectiva de la instrucción actual
                }
                Err(e) => {
                    eprintln!(
                        "Error al convertir la instrucción '{}' a binario: {}",
                        hex_string, e
                    );
                    break;
                }
            }
        }
    }

    /// Convierte una cadena hexadecimal en una cadena binaria
    fn hex_string_to_binary(&self, hex: &str) -> Result<String, String> {
        hex.chars()
            .map(|c| self.hex_char_to_binary(c))
            .collect::<Result<String, _>>() // Convierte el iterador a Result
    }

    /// Convierte un carácter hexadecimal en una cadena binaria
    fn hex_char_to_binary(&self, c: char) -> Result<String, String> {
        match c.to_digit(16) {
            Some(num) => Ok(format!("{:04b}", num)),
            None => Err(format!("Carácter no válido encontrado: '{}'", c)),
        }
    }

    /// Convierte una cadena binaria en un vector de enteros
    fn separar_binario_en_vector(&self, cadena: &str) -> Vec<i32> {
        cadena
            .chars()
            .filter_map(|c| {
                if c == '0' || c == '1' {
                    Some(c.to_digit(2).unwrap() as i32)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Extrae los componentes de una instrucción binaria
    fn extraer_componentes(&self, binario: &[u32]) -> (u32, u32, u32, u32, u32, u32) {
        if binario.len() < 32 {
            panic!("El vector binario debe contener al menos 32 bits.");
        }

        let cond = (binario[0] << 3) | (binario[1] << 2) | (binario[2] << 1) | binario[3]; // bits 31-28
        let i = binario[25]; // bit 25
        let opcode = (binario[21] << 3) | (binario[22] << 2) | (binario[23] << 1) | binario[24]; // bits 24-21
        let rn = (binario[16] << 3) | (binario[17] << 2) | (binario[18] << 1) | binario[19]; // bits 19-16
        let rd = (binario[12] << 3) | (binario[13] << 2) | (binario[14] << 1) | binario[15]; // bits 15-12
        let operand2 = (binario[0] << 11)
            | (binario[1] << 10)
            | (binario[2] << 9)
            | (binario[3] << 8)
            | (binario[4] << 7)
            | (binario[5] << 6)
            | (binario[6] << 5)
            | (binario[7] << 4)
            | (binario[8] << 3)
            | (binario[9] << 2)
            | (binario[10] << 1)
            | binario[11]; // bits 11-0

        (
            cond.try_into().unwrap(),
            i.try_into().unwrap(),
            opcode.try_into().unwrap(),
            rn.try_into().unwrap(),
            rd.try_into().unwrap(),
            operand2.try_into().unwrap(),
        )
    }
}
