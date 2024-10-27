/*
1.N (Negative flag): Indica si el resultado de una operación es negativo.
2.Z (Zero flag): Indica si el resultado de una operación es cero.
3.C (Carry flag): Indica si hubo un acarreo en una operación aritmética, lo que puede ocurrir en sumas o restas.
4.V (Overflow flag): Indica si ocurrió un desbordamiento en una operación aritmética, es decir, si el resultado excede la capacidad de representación del número.
*/

pub struct PlacaARM {
    keys: [bool; 4],    // Array de 4 teclas booleanas
    buttons: [bool; 2], // Array de 2 botones booleanos
    ledsP: [bool; 8],   // Array de 8 LEDs booleanos
    bits: [i32; 16],    // Array de 16 enteros
    flag: [bool; 4],    // Array de 4 banderas
}

impl PlacaARM {
    pub fn new() -> Self {
        PlacaARM {
            keys: [false; 4],
            buttons: [false; 2],
            ledsP: [false; 8],
            bits: [0; 16], // Inicializa el array de 16 enteros a 0
            flag: [false; 4], // Inicializa el array de banderas a false
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

    // Métodos para obtener y establecer números
    pub fn get_number(&self, index: usize) -> Option<i32> {
        if index < self.bits.len() {
            Some(self.bits[index])
        } else {
            None // Retorna None si el índice está fuera de rango
        }
    }

    pub fn set_number(&mut self, index: usize, value: i32) {
        if index < self.bits.len() {
            self.bits[index] = value;
        }
    }

    // Método para almacenar un valor en una dirección de memoria
    pub fn store(&mut self, value: i32, address: usize) {
        if address < self.bits.len() {
            self.bits[address] = value; // Almacena el valor en la dirección
        } else {
            println!("Error: dirección de memoria fuera de rango {}", address);
        }
    }


    // Métodos para obtener y establecer banderas
    pub fn get_flag(&self, index: usize) -> Option<bool> {
        if index < self.flag.len() {
            Some(self.flag[index])
        } else {
            None // Retorna None si el índice está fuera de rango
        }
    }

    pub fn set_flag(&mut self, index: usize, value: bool) {
        if index < self.flag.len() {
            self.flag[index] = value;
        }
    }

    // Método para obtener la cantidad de LEDs
    pub fn get_cantidad_leds(&self) -> usize {
        self.ledsP.len() // Retorna la longitud del array de LEDs
    }

    // Método para obtener el estado de un LED
    pub fn get_led(&self, index: usize) -> Option<bool> {
        if index < self.ledsP.len() {
            Some(self.ledsP[index])
        } else {
            None // Retorna None si el índice está fuera de rango
        }
    }

    // Método para establecer el estado de un LED
    pub fn set_led(&mut self, index: usize, value: bool) {
        if index < self.ledsP.len() {
            self.ledsP[index] = value;
        }
    }

 // Método para obtener un valor de la memoria
    pub fn get_memory(&self, address: usize) -> Option<i32> {
        if address < self.bits.len() {
            Some(self.bits[address]) // Devuelve el valor en la dirección especificada
        } else {
            None // Retorna None si la dirección está fuera de rango
        }
    }

    // Método para establecer un valor en la memoria
    pub fn set_memory(&mut self, address: usize, value: i32) {
        if address < self.bits.len() {
            self.bits[address] = value; // Almacena el valor en la dirección especificada
        } else {
            println!("Error: dirección de memoria fuera de rango {}", address);
        }
    }
}
