use std::{io::{self, Write}};
use crate::utilitis::archivos::archivo::Archivo; // Asegúrate de importar correctamente Archivo
use crate::utilitis::hardware::placa_arm::PlacaARM;
use crate::utilitis::traduccion::instruccionHex::InstruccionesHex;

pub struct Menu {}

impl Menu {
    pub fn new() -> Self {
        Menu {} // Constructor para la estructura Menu
    }

    pub fn mostrar(&self, placa: &mut PlacaARM) -> io::Result<()> {
        // Leer la elección del usuario
        let stdin = io::stdin();
        let mut archivo = Archivo::new("./utilitis/archivo.txt");
        let mut instruccionM = InstruccionesHex::new();

        loop {
            println!("\n###### Menú ######");
            println!("1. Leer archivo");
            println!("2. Sobrescribir archivo");
            println!("3. Agregar instrucciones");
            println!("4. Simular");
            println!("5. Salir");
            print!("Ingrese su opción: "); // Usar print en lugar de println

            io::stdout().flush()?; // Asegúrate de que se muestre antes de leer

            let mut entrada = String::new(); // Variable para almacenar la entrada del usuario

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
                            // Leer y mostrar instrucciones del archivo
                            if let Ok(_) = archivo.lectura_instrucciones() {
                                archivo.mostrar_instrucciones()?;
                            } else {
                                println!("Error al leer el archivo.");
                            }
                        }
                        2 => {
                            self.solicitar_y_escribir(&mut archivo, "Sobrescribir");
                        }
                        3 => {
                            self.solicitar_y_escribir(&mut archivo, "Agregar");
                        }
                        4 => {
                            instruccionM.convertir();
                        }
                        5 => {
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

    // Método para solicitar y escribir instrucciones en el archivo
    fn solicitar_y_escribir(&self, archivo: &mut Archivo, modo: &str) {
        println!("{} instrucciones...", modo);
        println!("\nIngrese las instrucciones nuevas:");
        print!("Ingrese las instrucciones: ");
        io::stdout().flush().unwrap(); // Asegúrate de que se imprima antes de leer

        let mut entrada = String::new(); // Crear una nueva variable para la entrada

        if io::stdin().read_line(&mut entrada).is_ok() {
            let entrada = entrada.trim(); // Eliminar salto de línea

            let resultado = if modo == "Sobrescribir" {
                archivo.sobrescribir(entrada)
            } else {
                archivo.agregar(entrada) // Si es "Agregar", llama al método de agregar
            };

            match resultado {
                Ok(_) => println!("Instrucciones guardadas correctamente."),
                Err(e) => println!("Error al escribir en el archivo: {}", e),
            }
        } else {
            println!("Error al leer la entrada.");
        }
    }
}
