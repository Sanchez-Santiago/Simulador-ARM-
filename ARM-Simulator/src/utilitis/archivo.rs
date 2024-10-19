// archivo.rs
use std::fs::File;
use std::io::{self, BufRead};

pub struct Archivo {
    texto: String,
}

impl Archivo {
    pub fn nueva(texto: &str) -> Archivo {
        Archivo {
            texto: String::from(texto),
        }
    }

    // Método para leer instrucciones y devolver un Vec de Vec<String>
    pub fn lectura_instrucciones(&self) -> io::Result<Vec<Vec<String>>> {
        // Abre el archivo
        let file = File::open(&self.texto)?;
        let reader = io::BufReader::new(file);

        // Crea un Vec para almacenar las instrucciones
        let mut instrucciones = Vec::new();

        // Itera sobre las líneas del archivo
        for line in reader.lines() {
            // Lee cada línea
            let line = line?;

            // Divide la línea en palabras (instrucción y argumentos) y las agrega al Vec
            let palabras: Vec<String> = line
                .split_whitespace() // Divide por espacios
                .map(|s| s.to_string()) // Convierte cada &str en String
                .collect(); // Recolecta en un Vec<String>

            // Agrega las palabras a las instrucciones
            instrucciones.push(palabras);
        }
        Ok(instrucciones)
    }
}