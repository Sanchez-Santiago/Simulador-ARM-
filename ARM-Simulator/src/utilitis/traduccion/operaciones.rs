use crate::utilitis::hardware::placa_arm::PlacaARM;

pub struct Operacion {}

impl Operacion {
    pub fn new() -> Self {
        Operacion {}
    }

    pub fn add(&self, placa: &mut PlacaARM, des: usize, x: usize, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x), placa.get_number(z as usize)) {
            placa.set_number(des, valor_x + valor_z);
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    pub fn sub(&self, placa: &mut PlacaARM, des: usize, x: usize, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x), placa.get_number(z as usize)) {
            placa.set_number(des, valor_x - valor_z);
            // Levantar banderas
            placa.set_flag(1, valor_x - valor_z == 0); // Bandera Z
            placa.set_flag(0, valor_x - valor_z < 0);  // Bandera N
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    pub fn subs(&self, placa: &mut PlacaARM, des: usize, x: usize, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x), placa.get_number(z as usize)) {
            placa.set_number(des, valor_x - valor_z);
            // Levanta las banderas
            placa.set_flag(1, valor_x - valor_z == 0); // Bandera Z
            placa.set_flag(0, valor_x - valor_z < 0);  // Bandera N
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    pub fn ldr(&self, placa: &mut PlacaARM, des: usize, dir: i32) {
        // Obtener el valor de la dirección de memoria
        if let Some(valor) = placa.get_number(dir as usize) {
            placa.set_number(des, valor);
        } else {
            println!("Error: índice fuera de rango o valor no encontrado en la dirección.");
        }
    }

    pub fn str(&self, placa: &mut PlacaARM, dir: i32, valor: i32) {
        // Almacenar el valor en la dirección de memoria
        if placa.get_number(dir as usize).is_some() {
            placa.set_number(dir as usize, valor);
        } else {
            println!("Error: índice fuera de rango o dirección de memoria no válida.");
        }
    }

    pub fn orr(&self, placa: &mut PlacaARM, des: usize, x: usize, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x), placa.get_number(z as usize)) {
            placa.set_number(des, valor_x | valor_z);
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    pub fn and(&self, placa: &mut PlacaARM, des: usize, x: usize, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x), placa.get_number(z as usize)) {
            placa.set_number(des, valor_x & valor_z);
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    // Método para el salto (B)
    pub fn b(&self, placa: &mut PlacaARM, offset: i32) {
        let pc_actual = placa.get_number(15).unwrap_or(0); // Suponiendo que el PC está en el índice 15
        let nueva_direccion = (pc_actual as i32 + offset) as usize;
        placa.set_number(15, nueva_direccion as i32); // Actualiza el PC
    }

    // Método para el salto condicional (BNE)
    pub fn bne(&self, placa: &mut PlacaARM, offset: i32) {
        if placa.get_flag(0).unwrap_or(false) == false { // Si la bandera N es 0
            self.b(placa, offset); // Realiza el salto
        }
    }

    // Método para el salto condicional (BEQ)
    pub fn beq(&self, placa: &mut PlacaARM, offset: i32) {
        if placa.get_flag(1).unwrap_or(false) == true { // Si la bandera Z es 1
            self.b(placa, offset); // Realiza el salto
        }
    }
}
