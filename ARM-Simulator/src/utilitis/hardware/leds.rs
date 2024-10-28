use super::placa_arm::PlacaARM;

pub struct Leds {}

impl Leds {
    pub fn new() -> Self {
        Leds {}
    }

    pub fn mostrar(&self, placa: &PlacaARM) {
        for i in 0..placa.get_cantidad_leds() {
            let estado = placa.get_led(i).unwrap_or(false);
            print!("({})", if estado { "o" } else { "-" });
        }
        println!();
    }

    pub fn encender(&mut self, placa: &mut PlacaARM, indice: usize) {
        if indice < placa.get_cantidad_leds() {
            placa.set_led(indice, true);
        } else {
            eprintln!("Índice fuera de rango: {}", indice);
        }
    }

    pub fn apagar(&mut self, placa: &mut PlacaARM, indice: usize) {
        if indice < placa.get_cantidad_leds() {
            placa.set_led(indice, false);
        } else {
            eprintln!("Índice fuera de rango: {}", indice);
        }
    }
}
