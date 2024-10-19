pub struct PlacaARM {
    keys: [bool; 4], // Array de 4 teclas booleanas
    buttons: [bool; 2], // Array de 2 botones booleanos
    ledsP: [bool; 8], // Array de 8 LEDs booleanos
    bits: [i16; 16], // Array de 16 enteros
}

impl PlacaARM {
    pub fn new() -> Self {
        // Inicializa la estructura con valores predeterminados
        PlacaARM {
            keys: [false; 4],
            buttons: [false; 2],
            ledsP: [false; 8],
            bits: [0; 16], // Inicializa el array de 16 enteros a 0
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
    pub fn get_number(&self, index: usize) -> Option<i16> {
        if index < self.bits.len() {
            Some(self.bits[index])
        } else {
            None // Retorna None si el índice está fuera de rango
        }
    }

    pub fn set_number(&mut self, index: usize, value: i16) {
        if index < self.bits.len() {
            self.bits[index] = value;
        }
    }

    // Método para obtener la cantidad de LEDs
    pub fn get_cantidad_leds(&self) -> usize {
        self.ledsP.len() // Retorna la longitud del array de LEDs
    }
}
