use crate::utilitis::{archivos::archivo::Archivo, traduccion::binariToOperaciones};

pub struct Traductor {}

impl Traductor {
    pub fn new() -> Self {
        Traductor {}
    }

    pub fn convertir(&self) {
        let archivo = Archivo::new("./src/utilitis/archivos/imem_io.dat");
        let operaciones_bin = binariToOperaciones::BinarioToOperacion::new();
        let instrucciones_h = archivo
            .lectura_instrucciones()
            .expect("Error al leer instrucciones"); // Obtener las instrucciones

        for linea in instrucciones_h {
            // Asegúrate de que `linea` sea un String
            //let binary_representation = self.hex_string_to_binary(&linea); // Aquí se debe pasar un &String o &str
            //let binari_vec = self.separar_binario_en_vector(&binary_representation);

            // Aquí puedes pasar el vector binario a la clase Operacion
            // Por ejemplo, si quieres realizar una operación como ADD:
            // operaciones_bin.add(&mut placa_arm, dest, src1, src2);
            // Reemplaza `dest`, `src1` y `src2` con los índices apropiados

            // Solo para mostrar el vector binario
            //println!("Instrucción: {}, Vector Binario: {:?}", linea, binari_vec);
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
}
