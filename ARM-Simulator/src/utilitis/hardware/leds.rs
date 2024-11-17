use crate::utilitis::hardware::placa_arm::PlacaARM;

pub struct Leds {
    leds: [bool; 8],
}

impl Leds {
    pub fn new() -> Self {
        Leds {
            leds: [false; 8],
        }
    }

    pub fn mostrar(&mut self, valor: i32) {
        // Convertir el valor a un número binario de 8 bits
        let valor_binario = format!("{:08b}", valor & 0xFF); // Asegurarse de que sólo se usan los 8 bits menos significativos

        for (i, bit) in valor_binario.chars().enumerate() {
            let estado = bit == '1'; // Si el bit es 1, el LED debe estar encendido
            self.leds[i] = estado;
            print!("({})", if estado { "o" } else { " " });
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
