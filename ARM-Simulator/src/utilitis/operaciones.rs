use super::{placa_arm::PlacaARM};
use std::io;

pub struct Operacion{}

impl Operacion {

    pub fn new() -> Self {
        Operacion {}
    }

    pub fn add(&self, placa: &mut PlacaARM, des: usize, x: usize, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x), placa.get_number(z as usize)) {
            placa.set_number(des, valor_x + valor_z);
            // Levantar banderas
            placa.set_flag(1, valor_x + valor_z == 0); // Bandera Z
            placa.set_flag(0, valor_x + valor_z < 0); // Bandera N
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    pub fn sub(&self, placa: &mut PlacaARM, des: usize, x: usize, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x), placa.get_number(z as usize)) {
            placa.set_number(des, valor_x - valor_z);
            // Levantar banderas
            placa.set_flag(1, valor_x - valor_z == 0); // Bandera Z
            placa.set_flag(0, valor_x - valor_z < 0); // Bandera N
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    pub fn subs(&self, placa: &mut PlacaARM, des: usize, x: usize, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x), placa.get_number(z as usize)) {
            placa.set_number(des, valor_x - valor_z);
            // Levanta las banderas
            placa.set_flag(1, valor_x - valor_z == 0); // Bandera Z
            placa.set_flag(0, valor_x - valor_z < 0); // Bandera N
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }
}