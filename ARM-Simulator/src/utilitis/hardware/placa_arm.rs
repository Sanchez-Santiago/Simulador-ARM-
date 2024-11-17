pub struct PlacaARM {
    keys: [bool; 4],              // Array de 4 teclas booleanas
    buttons: [bool; 2],           // Array de 2 botones booleanos
    leds: [bool; 8],              // Array de 8 LEDs booleanos
    registros: [i32; 16],         // Array de 16 registros
    flags: [bool; 4],             // Array de 4 banderas
    memoria: Vec<i32>,            // Memoria dinámica de 2 MB
}
// flags:
// 0	V	Desbordamiento aritmético
// 1	C	Acarreo o préstamo
// 2	Z	Resultado igual a cero
// 3	N	Resultado negativo

impl PlacaARM {
    pub fn new() -> Self {
        PlacaARM {
            keys: [false; 4],
            buttons: [false; 2],
            leds: [false; 8],
            registros: [0; 16],
            flags: [false; 4],
            memoria: vec![0; 524_288], // Inicializa con 524_288 elementos en 0
        }
    }

    // Métodos para obtener valores de las teclas
    pub fn get_key(&self, index: usize) -> Option<bool> {
        if index < self.keys.len() {
            Some(self.keys[index])
        } else {
            None // Retorna None si el índice está fuera de rango
        }
    }

    // Métodos para establecer valores de las teclas
    pub fn set_key(&mut self, index: usize, value: bool) {
        if index < self.keys.len() {
            self.keys[index] = value;
        }
    }

    // Métodos para obtener y establecer registros
    pub fn get_register(&self, index: usize) -> Option<i32> {
        if index < self.registros.len() {
            Some(self.registros[index])
        } else {
            None // Retorna None si el índice está fuera de rango
        }
    }

    pub fn set_register(&mut self, index: usize, value: i32) {
        if index < self.registros.len() {
            self.registros[index] = value;
        }
    }

    // Métodos para obtener y establecer banderas
    pub fn get_flag(&self, index: usize) -> Option<bool> {
        if index < self.flags.len() {
            Some(self.flags[index])
        } else {
            None // Retorna None si el índice está fuera de rango
        }
    }

    pub fn set_flag(&mut self, index: usize, value: bool) {
        if index < self.flags.len() {
            self.flags[index] = value;
        }
    }

    // Método para obtener la cantidad de LEDs
    pub fn get_cantidad_leds(&self) -> usize {
        self.leds.len() // Retorna la longitud del array de LEDs
    }

    // Método para obtener el estado de un LED
    pub fn get_led(&self, index: usize) -> Option<bool> {
        if index < self.leds.len() {
            Some(self.leds[index])
        } else {
            None // Retorna None si el índice está fuera de rango
        }
    }

    // Método para establecer el estado de un LED
    pub fn set_led(&mut self, index: usize, value: bool) {
        if index < self.leds.len() {
            self.leds[index] = value;
        }
    }

    // Método para obtener un valor de un registro
    pub fn get_register_memory(&self, address: usize) -> Option<i32> {
        if address < self.registros.len() {
            Some(self.registros[address]) // Devuelve el valor en la dirección especificada
        } else {
            None // Retorna None si la dirección está fuera de rango
        }
    }

    // Método para establecer un valor en un registro
    pub fn set_register_memory(&mut self, address: usize, value: i32) {
        if address < self.registros.len() {
            self.registros[address] = value; // Almacena el valor en la dirección especificada
        } else {
            println!("Error: dirección de memoria fuera de rango {}", address);
        }
    }

    // Método para obtener un valor de la memoria principal de 2 MB
    pub fn get_memory(&self, address: usize) -> Option<i32> {
        self.memoria.get(address).copied() // Usa get para evitar accesos fuera de rango
    }

    // Método para establecer un valor en la memoria principal de 2 MB
    pub fn set_memory(&mut self, address: usize, value: i32) {
        if let Some(slot) = self.memoria.get_mut(address) {
            *slot = value;
        } else {
            println!("Error: dirección de memoria fuera de rango {}", address);
        }
    }
}
