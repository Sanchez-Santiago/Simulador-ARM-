use super::placa_arm::PlacaARM;

pub struct Leds {}

impl Leds {
    pub fn new() -> Self {
        Leds {} // Constructor de Leds sin necesidad de placa
    }

    // Método mostrar que recibe una referencia a PlacaARM
    pub fn mostrar(&self, placa: &PlacaARM) {
        // Muestra el estado de los LEDs usando el estado de la placa
        for i in 0..placa.ledsP.len() {
            let estado = placa.get_led(i).unwrap_or(false); // Obtiene el estado del LED
            print!("({})", if estado { "o" } else { "-" });
        }
        println!(); // Nueva línea después de mostrar el estado de los LEDs
    }

    // Métodos para encender y apagar LEDs
    pub fn encender(&mut self, placa: &mut PlacaARM, indice: usize) {
        placa.set_led(indice, true); // Enciende el LED en el índice dado
    }

    pub fn apagar(&mut self, placa: &mut PlacaARM, indice: usize) {
        placa.set_led(indice, false); // Apaga el LED en el índice dado
    }
}
