
// archivo.rs
use std::fs;
use std::io;

pub struct Archivo {
    pub texto: String,
}

impl Archivo {
    // Método de instancia para leer el archivo
    pub fn nueva(texto: &str) -> Archivo {
        Archivo {
            texto: String::from(texto),
        }
    }

    pub fn lectura(&self) -> io::Result<String> {
        let contenido = fs::read_to_string(&self.texto)?;
        Ok(contenido)
    }
}
