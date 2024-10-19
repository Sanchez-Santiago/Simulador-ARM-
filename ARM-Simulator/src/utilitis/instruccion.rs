use super::{archivo::Archivo, leds::{self, Leds}, placa_arm::PlacaARM};
use std::io;

pub struct Instrucciones {}

impl Instrucciones {
    pub fn new() -> Self {
        Instrucciones {}
    }

    pub fn traducir(&self, placa: &mut PlacaARM) -> io::Result<()> {
        let archivo = Archivo::nueva("./utilitis/archivo.txt");
        let instrucciones = archivo.lectura_instrucciones()?; // Obtener las instrucciones
        let led= leds::Leds::new();

        for instruccion in instrucciones {
            if instruccion.is_empty() {
                continue; // Saltar líneas vacías
            }

            // Ejemplo: procesar la primera palabra como la instrucción
            let operacion = &instruccion[0];

            match operacion.as_str() {
                "ADD" => {
                    if instruccion.len() < 4 {
                        println!("Error: se requieren 3 argumentos para ADD");
                        continue;
                    }
                    let des = self.extract_register_index(&instruccion[1]);
                    let x = self.extract_register_index(&instruccion[2]);
                    let z = self.extract_value(&instruccion[3]);
                    self.add(placa, des, x, z);
                }
                "SUB" => {
                    if instruccion.len() < 4 {
                        println!("Error: se requieren 3 argumentos para SUB");
                        continue;
                    }
                    let des = self.extract_register_index(&instruccion[1]);
                    let x = self.extract_register_index(&instruccion[2]);
                    let z = self.extract_value(&instruccion[3]);
                    self.sub(placa, des, x, z);
                }
                "STR" => {
                    // Lógica para STR
                    if instruccion.len() < 3 {
                        println!("Error: se requieren al menos 2 argumentos para STR");
                        continue;
                    }
                    let valor = self.extract_register_index(&instruccion[1]);
                    let destino = self.extract_memory_address(&instruccion[2]);
                    // Implementar lógica para almacenar el valor en la dirección
                }
                "BNE" => {
                    // Lógica para BNE
                    if instruccion.len() < 2 {
                        println!("Error: se requiere un argumento para BNE");
                        continue;
                    }
                    let label = &instruccion[1];
                    // Implementar lógica para la instrucción BNE
                }
                "B" => {
                    // Lógica para B
                    if instruccion.len() < 2 {
                        println!("Error: se requiere un argumento para B");
                        continue;
                    }
                    let label = &instruccion[1];
                    // Implementar lógica para la instrucción B
                }
                _ => {
                    println!("Instrucción no reconocida: {}", operacion);
                }
            }
            led
        }
        Ok(())
    }

    fn extract_register_index(&self, register: &str) -> usize {
        // Extrae el índice del registro de la forma "R0", "R1", etc.
        register.trim_start_matches('R').parse().unwrap_or_else(|_| {
            println!("Error: registro no válido {}", register);
            0 // Devolver un valor por defecto o manejar el error de otra manera
        })
    }

    fn extract_value(&self, value: &str) -> i32 {
        // Lógica para extraer un valor numérico de la forma "#1" o "#0x100000"
        if value.starts_with('#') {
            if let Ok(val) = i32::from_str_radix(&value[1..], 16) {
                return val;
            }
            return value[1..].parse().unwrap_or(0); // En caso de que no sea hex
        }
        // Si no es un valor inmediato, debe ser un registro
        self.extract_register_index(value)
    }

    fn extract_memory_address(&self, address: &str) -> usize {
        // Extrae la dirección de memoria de la forma "[R0, #0X800]" o "[R0]"
        if address.starts_with('[') && address.ends_with(']') {
            // Eliminar los corchetes
            let inner = &address[1..address.len() - 1].trim();
            // Manejar la lógica para obtener el registro y el desplazamiento
            let parts: Vec<&str> = inner.split(',').collect();
            let register_index = self.extract_register_index(parts[0]);
            let offset = if parts.len() > 1 {
                self.extract_value(parts[1].trim())
            } else {
                0
            };
            return register_index + offset as usize; // Ejemplo de cómo combinar
        }
        0 // Valor por defecto si no se puede procesar
    }

    fn add(&self, placa: &mut PlacaARM, des: usize, x: usize, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x), placa.get_number(z as usize)) {
            placa.set_number(des, valor_x + valor_z);
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    fn sub(&self, placa: &mut PlacaARM, des: usize, x: usize, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x), placa.get_number(z as usize)) {
            placa.set_number(des, valor_x - valor_z);
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }
}
