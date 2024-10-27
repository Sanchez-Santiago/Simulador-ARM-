use crate::utilitis::hardware::{leds::{self, Leds}, placa_arm::PlacaARM};

pub struct Operacion {}

impl Operacion {
    pub fn new() -> Self {
        Operacion {}
    }

    pub fn operar(
        &self,
        placa: &mut PlacaARM,
        des: i32,
        x: i32,
        z: i32,
        es_inmediato: bool,
        bit_s: bool,
        operacion: fn(i32, i32) -> i32,
    ) {
        match placa.get_number(x as usize) {
            Some(valor_x) => {
                let valor_z = if es_inmediato { z } else { placa.get_number(z as usize).unwrap_or(0) };
                let resultado = operacion(valor_x, valor_z);
                placa.set_number(des as usize, resultado);

                if bit_s {
                    placa.set_flag(1, resultado == 0); // Bandera Z
                    placa.set_flag(0, resultado < 0);  // Bandera N

                    let (resultado_c, carry) = match operacion {
                        f if f == |a, b| a + b => (valor_x as u32).overflowing_add(valor_z as u32),
                        f if f == |a, b| a - b => (valor_x as u32).overflowing_sub(valor_z as u32),
                        _ => (0, false),
                    };
                    placa.set_flag(2, carry); // Bandera C

                    let overflow = match operacion {
                        f if f == |a, b| a + b => (valor_x > 0 && valor_z > 0 && resultado < 0) || (valor_x < 0 && valor_z < 0 && resultado > 0),
                        f if f == |a, b| a - b => (valor_x < 0 && valor_z > 0 && resultado > 0) || (valor_x > 0 && valor_z < 0 && resultado < 0),
                        _ => false,
                    };
                    placa.set_flag(3, overflow); // Bandera V
                }
            }
            None => println!("Error: índice fuera de rango o valor no encontrado para el índice {}", x),
        }
    }

    pub fn add(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32, es_inmediato: bool, bit_s: bool) {
        self.operar(placa, des, x, z, es_inmediato, bit_s, |a, b| a + b);
    }

    pub fn sub(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32, es_inmediato: bool, bit_s: bool) {
        self.operar(placa, des, x, z, es_inmediato, bit_s, |a, b| a - b);
    }

    pub fn sbc(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x as usize), placa.get_number(z as usize)) {
            let carry = placa.get_flag(2).unwrap_or(false) as i32;
            let resultado = valor_x - valor_z - (1 - carry);
            placa.set_number(des as usize, resultado);
            placa.set_flag(1, resultado == 0); // Bandera Z
            placa.set_flag(0, resultado < 0);  // Bandera N
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    pub fn orr(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32, es_inmediato: bool, bit_s: bool) {
        self.operar(placa, des, x, z, es_inmediato, bit_s, |a, b| a | b);
    }

    pub fn and(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32, es_inmediato: bool, bit_s: bool) {
        self.operar(placa, des, x, z, es_inmediato, bit_s, |a, b| a & b);
    }

    pub fn eor(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32, es_inmediato: bool, bit_s: bool) {
        self.operar(placa, des, x, z, es_inmediato, bit_s, |a, b| a ^ b);
    }

    pub fn bic(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32, es_inmediato: bool, bit_s: bool) {
        self.operar(placa, des, x, z, es_inmediato, bit_s, |a, b| a & !b);
    }

    pub fn mvn(&self, placa: &mut PlacaARM, des: i32, z: i32, es_inmediato: bool) {
        let valor_z = if es_inmediato { z } else { placa.get_number(z as usize).unwrap_or(0) };
        placa.set_number(des as usize, !valor_z);
        placa.set_flag(1, !valor_z == 0); // Bandera Z
        placa.set_flag(0, !valor_z < 0);  // Bandera N
    }

    pub fn mov(&self, placa: &mut PlacaARM, des: i32, z: i32, es_inmediato: bool) {
        let valor_z = if es_inmediato { z } else { placa.get_number(z as usize).unwrap_or(0) };
        placa.set_number(des as usize, valor_z);
        placa.set_flag(1, valor_z == 0); // Bandera Z
        placa.set_flag(0, valor_z < 0);  // Bandera N
    }

    pub fn tst(&self, placa: &mut PlacaARM, x: i32, z: i32, es_inmediato: bool) {
        let valor_x = placa.get_number(x as usize).unwrap_or(0);
        let valor_z = if es_inmediato { z } else { placa.get_number(z as usize).unwrap_or(0) };
        let resultado = valor_x & valor_z;
        placa.set_flag(1, resultado == 0); // Bandera Z
        placa.set_flag(0, resultado < 0);  // Bandera N
    }

    pub fn cmp(&self, placa: &mut PlacaARM, x: i32, z: i32, es_inmediato: bool) {
        let valor_x = placa.get_number(x as usize).unwrap_or(0);
        let valor_z = if es_inmediato { z } else { placa.get_number(z as usize).unwrap_or(0) };
        let resultado = valor_x - valor_z;
        placa.set_flag(1, resultado == 0); // Bandera Z
        placa.set_flag(0, resultado < 0);  // Bandera N
        let carry = (valor_x as u32).overflowing_sub(valor_z as u32).1;
        placa.set_flag(2, carry); // Bandera C
        let overflow = (valor_x < 0 && valor_z > 0 && resultado > 0) || (valor_x > 0 && valor_z < 0 && resultado < 0);
        placa.set_flag(3, overflow); // Bandera V
    }

      // Método para restar con signo inverso
    pub fn rsb(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x as usize), placa.get_number(z as usize)) {
            let resultado = valor_z - valor_x;
            placa.set_number(des as usize, resultado);
            // Levantar banderas
            placa.set_flag(1, resultado == 0); // Bandera Z
            placa.set_flag(0, resultado < 0);  // Bandera N
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    // Método para sumar con acarreo
    pub fn adc(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x as usize), placa.get_number(z as usize)) {
            let carry = placa.get_flag(2).unwrap_or(false) as i32; // Supone que carry está en la posición 2
            let resultado = valor_x + valor_z + carry;
            placa.set_number(des as usize, resultado);
            // Levantar banderas
            placa.set_flag(1, resultado == 0); // Bandera Z
            placa.set_flag(0, resultado < 0);  // Bandera N
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    // Método para restar con acarreo inverso
    pub fn rsc(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x as usize), placa.get_number(z as usize)) {
            let carry = placa.get_flag(2).unwrap_or(false) as i32; // Supone que carry está en la posición 2
            let resultado = valor_z - valor_x - (1 - carry);
            placa.set_number(des as usize, resultado);
            // Levantar banderas
            placa.set_flag(1, resultado == 0); // Bandera Z
            placa.set_flag(0, resultado < 0);  // Bandera N
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    // Método para comparar valores
    pub fn teq(&self, placa: &mut PlacaARM, x: i32, z: i32, es_inmediato: bool) {
        let valor_x = placa.get_number(x as usize).unwrap_or(0);
        let valor_z = if es_inmediato { z } else { placa.get_number(z as usize).unwrap_or(0) };

        let resultado = valor_x ^ valor_z;

        // Levantar banderas
        placa.set_flag(1, resultado == 0); // Bandera Z
        placa.set_flag(0, resultado < 0);  // Bandera N
    }

    pub fn cmn(&self, placa: &mut PlacaARM, x: i32, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_number(x as usize), placa.get_number(z as usize)) {
            let resultado = valor_x + valor_z;

            // Levantar banderas
            placa.set_flag(1, resultado == 0); // Bandera Z
            placa.set_flag(0, resultado < 0);  // Bandera N
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    // Método para manejar la interrupción de software (SWI)
    pub fn swi(&self, placa: &mut PlacaARM) {
        // Aquí puedes definir la lógica para manejar la interrupción de software
        // Por ejemplo, puedes establecer un flag o ejecutar una rutina de servicio
        // Este es solo un ejemplo básico
        //placa.trigger_software_interrupt(); // Método ficticio, implementa lo que necesites aquí

        println!("Interrupción de software ejecutada.");
    }

    pub fn bl(&mut self, placa: &mut PlacaARM, operand2: i32) {
        // Lógica para Branch with Link
        println!("Realizando Branch with Link a la dirección: {}", operand2);
        // Almacenar la dirección de retorno en un registro según sea necesario
    }

    pub fn str(&mut self, rd: i32, rn: i32, operand2: i32, placa: &mut PlacaARM) {
        // Lógica para almacenar el registro en memoria
        let direccion = rn + operand2; // Ajusta según sea necesario
        println!("Almacenando el valor del registro {} en la dirección {}", rd, direccion);
        // Implementa la lógica para almacenar el valor del registro en memoria
        let valor_a_almacenar = placa.get_register(rd); // Método hipotético para obtener el valor de un registro
        placa.store_memory(direccion, valor_a_almacenar); // Método hipotético para almacenar en memoria
    }

    pub fn ldr(&mut self, rd: i32, rn: i32, operand2: i32, placa: &mut PlacaARM) {
        // Lógica para cargar un registro
        let direccion = rn + operand2; // Ajusta esta lógica según tu arquitectura
        let valor_cargado = placa.load_memory(direccion); // Método para cargar desde memoria

        println!("Cargando el valor de la dirección {} en el registro {}", direccion, rd);
        placa.set_register(rd, valor_cargado); // Método para establecer el valor de un registro
    }
}
