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

        let mut pc: i32 = 0;
        placa.set_register(15, pc + 8); // Inicializar PC+8 para la primera instrucción

        while (pc / 4) < instrucciones_h.len().try_into().unwrap() {
            let current_pc = pc as usize;

            // Validar índice antes de acceder
            if current_pc / 4 >= instrucciones_h.len() {
                eprintln!(
                    "Índice fuera de rango: pc={}, current_pc={}, len={}",
                    pc,
                    current_pc,
                    instrucciones_h.len()
                );
                break;
            }

            // Leer la instrucción hexadecimal
            let hex_string: String = instrucciones_h[current_pc / 4].iter().collect();

            // Convertir a binario y procesar
            match self.hex_string_to_binary( &hex_string) {
                Ok(binario_str) => {
                    let bits = self.separar_binario_en_vector(&binario_str);

                    //println!(
                    //    "Ejecutando instrucción en PC=0x{:08X} (índice={}): {}",
                    //    pc,
                    //   current_pc / 4,
                    //   hex_string
                    //);

                    // Ejecutar la instrucción (R15 ya contiene PC+8)
                    instrucciones_bina.llamado(
                        &bits.iter().map(|&b| b as i32).collect::<Vec<i32>>(),
                        placa,
                    );
                }
                Err(e) => {
                    eprintln!(
                        "Error al convertir la instrucción en PC=0x{:08X} ({}): {}",
                        pc, hex_string, e
                    );
                    break;
                }
            }

            // Obtener el nuevo PC después de la ejecución
            match placa.get_register(15) {
                Some(next_pc) => {
                    if next_pc <= 0 || (next_pc / 4) as usize >= (instrucciones_h.len() + 2) {
                        eprintln!("Valor inválido del PC: {}", next_pc);
                        break;
                    }
                    if next_pc% 4 != 0{
                        pc = next_pc -2 ; // Ajustar el PC si hubo salto
                    }else {
                        // Manejar casos de salto y avance lineal
                        if next_pc == pc + 8  {
                            pc += 4; // Avanzar a la siguiente instrucción
                        } else {
                            pc = next_pc ; // Ajustar el PC si hubo salto
                        }
                    }

                    
                    // Validar alineación del PC
                    if pc % 4 != 0 {
                        eprintln!("Error: PC desalineado ({}). Ajustando.", pc);
                        pc = (pc / 4) * 4;
                    }

                    // Actualizar R15 con PC+8
                    placa.set_register(15, pc + 8);
                }
                None => {
                    eprintln!("Error: El valor del PC (R15) es None.");
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
}
