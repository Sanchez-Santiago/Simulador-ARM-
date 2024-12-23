use std::fs::{self, File, OpenOptions}; // Asegúrate de que File esté importado
use std::io::{self, BufRead, Write};

pub struct Archivo {
    texto: String,
}

impl Archivo {
    pub fn new(texto: &str) -> Archivo {
        Archivo {
            texto: String::from(texto),
        }
    }

    // Método para leer instrucciones y devolver un Vec de Vec<char>
    pub fn lectura_instrucciones(&self) -> io::Result<Vec<Vec<char>>> {
        // Abre el archivo
        let file = File::open(&self.texto)?;
        let reader = io::BufReader::new(file);

        // Crea un Vec para almacenar las instrucciones
        let mut instrucciones = Vec::new();

        // Itera sobre las líneas del archivo
        for line in reader.lines() {
            // Lee cada línea
            let line = line?;
            
            // Convierte cada línea en un Vec<char>
            let caracteres: Vec<char> = line.chars().collect(); // Convierte cada carácter en un Vec<char>

            // Agrega los caracteres a las instrucciones
            instrucciones.push(caracteres);
        }
        Ok(instrucciones)
    }

    // Método para mostrar instrucciones
    pub fn mostrar_instrucciones(&self) -> io::Result<()> {
        println!("\nInstrucciones:");
        let file = File::open(&self.texto)?;
        let reader = io::BufReader::new(file);

        // Itera sobre las líneas del archivo
        for line in reader.lines() {
            // Lee cada línea
            let line = line?;
            println!("{}", line); // Imprime cada línea
        }
        Ok(())
    }

    // Método para sobrescribir el archivo con nuevo contenido
    pub fn sobrescribir(&self, nuevo_contenido: &str) -> io::Result<()> {
        let mut file = File::create(&self.texto)?; // Crea o sobrescribe el archivo
        file.write_all(nuevo_contenido.as_bytes())?; // Escribe el nuevo contenido
        Ok(())
    }

    // Método para agregar texto al final del archivo
    pub fn agregar(&self, texto_adicional: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true) // Habilita el modo de agregar
            .open(&self.texto)?; // Abre el archivo
        file.write_all(texto_adicional.as_bytes())?; // Escribe el texto adicional
        Ok(())
    }
}
