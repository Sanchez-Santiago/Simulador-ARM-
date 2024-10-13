use std::io::{self, Write}; // Importar Write para poder usar flush

pub struct Menu {}

impl Menu {
    pub fn new() -> Self {
        Menu {} // Constructor para la estructura Menu
    }

    pub fn mostrar(&self, contenido: &str) -> io::Result<()> {
        // Leer la elección del usuario
        let stdin = io::stdin();

        loop {
            println!("###### Menú ######");
            println!("1. Leer archivo");
            println!("2. Escribir archivo");
            println!("3. Buscar palabra");
            println!("4. Salir");
            print!("Ingrese su opción: "); // Usar print en lugar de println

            io::stdout().flush()?; // Asegúrate de que se muestre antes de leer

            let mut entrada = String::new(); // Declarar aquí

            // Leer la línea de entrada
            if stdin.read_line(&mut entrada).is_err() {
                println!("Error al leer la entrada. Intente de nuevo.");
                continue; // Volver a comenzar el bucle
            }

            // Intentar convertir la entrada a un número
            match entrada.trim().parse::<u32>() {
                Ok(opcion) => {
                    match opcion {
                        1 => {
                            println!("Leyendo archivo...");
                            println!("{}", contenido);
                        }
                        2 => {
                            println!("Escribiendo archivo...");
                            // Implementar código para escribir contenido en el archivo
                        }
                        3 => {
                            println!("Buscando palabra...");
                            // Implementar código para buscar palabra en el contenido del archivo
                        }
                        4 => {
                            println!("Saliendo...");
                            break; // Salir del bucle
                        }
                        _ => {
                            println!("Opción no válida. Intente de nuevo.");
                        }
                    }
                }
                Err(_) => {
                    println!("Por favor, ingrese un número válido."); // Mensaje de error al fallar la conversión
                }
            }
        }

        Ok(())
    }
}
