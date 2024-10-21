use super::{archivo::Archivo, leds::{self, Leds}, placa_arm::PlacaARM, operaciones::Operacion};
use std::io;
use std::collections::HashMap;

pub struct Instrucciones {}

impl Instrucciones {
    pub fn new() -> Self {
        Instrucciones {}
    }

    pub fn traducir(&self, placa: &mut PlacaARM) -> io::Result<()> {
        let operaciones = Operacion::new();
        let archivo = Archivo::new("./utilitis/archivo.txt");
        let instrucciones = archivo.lectura_instrucciones()?; // Obtener las instrucciones
        let led = leds::Leds::new();

        // Preprocesar para obtener las etiquetas y sus índices
        let label_map = self.get_label_index_map(&instrucciones);

        let mut index = 0; // Inicializamos el índice
        while index < instrucciones.len() {
            let instruccion = &instrucciones[index]; // Accede a la instrucción en el índice actual

            if instruccion.is_empty() {
                index += 1; // Saltar líneas vacías
                continue;
            }

            // Procesar la primera palabra como la instrucción
            let operacion = &instruccion[0];

            match operacion.as_str() {
                "ADD" => {
                    if instruccion.len() < 4 {
                        println!("Error: se requieren 3 argumentos para ADD");
                        index += 1; // Mueve al siguiente índice
                        continue;
                    }
                    let des = self.extract_register_index(&instruccion[1]);
                    let x = self.extract_register_index(&instruccion[2]);
                    let z = self.extract_value(&instruccion[3]);
                    operaciones.add(placa, des, x, z);
                }
                "SUB" => {
                    if instruccion.len() < 4 {
                        println!("Error: se requieren 3 argumentos para SUB");
                        index += 1;
                        continue;
                    }
                    let des = self.extract_register_index(&instruccion[1]);
                    let x = self.extract_register_index(&instruccion[2]);
                    let z = self.extract_value(&instruccion[3]);
                    operaciones.sub(placa, des, x, z);
                }
                "SUBS" => {
                    if instruccion.len() < 4 {
                        println!("Error: se requieren 3 argumentos para SUBS");
                        index += 1;
                        continue;
                    }
                    let des = self.extract_register_index(&instruccion[1]);
                    let x = self.extract_register_index(&instruccion[2]);
                    let z = self.extract_value(&instruccion[3]);
                    operaciones.subs(placa, des, x, z);
                }
                "STR" => {
                    if instruccion.len() < 3 {
                        println!("Error: se requieren al menos 2 argumentos para STR");
                        index += 1;
                        continue;
                    }
                    let valor = self.extract_register_index(&instruccion[1]);
                    let destino = self.extract_memory_address(&instruccion[2]);

                    // Lógica para almacenar el valor en la dirección
                    placa.store(valor as i16, destino);

                    // Verificar si el destino es 0x800 y encender los LEDs
                    if destino == 0x800 {
                        // Enciende todos los LEDs como ejemplo
                        for i in 0..placa.get_cantidad_leds() {
                            placa.set_led(i, true); // Enciende el LED en la posición i
                        }
                        led.mostrar(placa);
                    }
                }

                "BNE" => {
                    if instruccion.len() < 2 {
                        println!("Error: se requiere un argumento para BNE");
                        index += 1;
                        continue;
                    }
                    let label = &instruccion[1];

                    // Aquí debes verificar si la condición se cumple
                    if !placa.get_flag(1).unwrap_or(false) { // Suponiendo que el flag Z está en la posición 1
                        if let Some(branch_index) = label_map.get(label) {
                            index = *branch_index; // Salta a la instrucción en el índice de la etiqueta
                            continue;
                        } else {
                            println!("Etiqueta no encontrada: {}", label);
                        }
                    }
                }
                "BEQ" => {
                    if instruccion.len() < 2 {
                        println!("Error: se requiere un argumento para BEQ");
                        index += 1;
                        continue;
                    }
                    let label = &instruccion[1];

                    // Aquí debes verificar si la condición se cumple
                    if placa.get_flag(1).unwrap_or(true) { // Suponiendo que el flag Z está en la posición 1
                        if let Some(branch_index) = label_map.get(label) {
                            index = *branch_index; // Salta a la instrucción en el índice de la etiqueta
                            continue;
                        } else {
                            println!("Etiqueta no encontrada: {}", label);
                        }
                    }
                }
                "B" => {
                    if instruccion.len() < 2 {
                        println!("Error: se requiere un argumento para B");
                        index += 1;
                        continue;
                    }
                    let label = &instruccion[1];

                    if let Some(branch_index) = label_map.get(label) {
                        index = *branch_index; // Salta a la instrucción en el índice de la etiqueta
                        continue; // Continúa con el siguiente ciclo
                    } else {
                        println!("Etiqueta no encontrada: {}", label);
                    }
                }
                _ => {
                    println!("Instrucción no reconocida: {}", operacion);
                }
            }
            index += 1; // Avanza al siguiente índice
        }
        Ok(())
    }

    fn extract_register_index(&self, register: &str) -> usize {
        // Verifica que el registro no tenga comas o espacios
        let reg_cleaned = register.trim().trim_end_matches(','); // Elimina comas y espacios

        // Si el registro comienza con 'R', lo procesamos normalmente
        if reg_cleaned.starts_with('R') {
            if let Ok(index) = reg_cleaned[1..].parse::<usize>() {
                return index;
            }
        }

        // Manejar registros hexadecimales
        if reg_cleaned.starts_with("0x") || reg_cleaned.starts_with("0X") {
            if let Ok(index) = usize::from_str_radix(&reg_cleaned[2..], 16) {
                return index;
            }
        }

        println!("Error: registro no válido {}", register);
        0 // Devolver un valor por defecto o manejar el error de otra manera
    }


    fn extract_value(&self, value: &str) -> i32 {
        if value.starts_with('#') {
            let val_str = &value[1..]; // Eliminar el símbolo de #
            if val_str.starts_with("0x") || val_str.starts_with("0X") {
                // Si es hexadecimal, intenta convertir
                if let Ok(val) = i32::from_str_radix(&val_str[2..], 16) {
                    return val;
                } else {
                    println!("Error: valor hexadecimal no válido {}", value);
                    return 0; // O manejar de otra forma
                }
            } else {
                // Si no es hexadecimal, intenta convertir como decimal
                return val_str.parse().unwrap_or_else(|_| {
                    println!("Error: valor no válido {}", value);
                    0 // Devolver un valor por defecto o manejar el error de otra manera
                });
            }
        }
        
        // Si no es un valor inmediato, debe ser un registro
        self.extract_register_index(value) as i32
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

    fn get_label_index_map(&self, instrucciones: &[Vec<String>]) -> HashMap<String, usize> {
        let mut label_map: HashMap<String, usize> = HashMap::new();
        
        for (index, instruccion) in instrucciones.iter().enumerate() {
            if instruccion.len() > 0 {
                if let Some(lbl) = instruccion.get(0) {
                    if lbl.ends_with(':') {
                        // Solo guardar etiquetas sin el ':'
                        label_map.insert(lbl.trim_end_matches(':').to_string(), index);
                    }
                }
            }
        }

        label_map // Devuelve el mapa de etiquetas
    }

}