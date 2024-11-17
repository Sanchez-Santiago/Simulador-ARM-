use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::utilitis::hardware::{leds::{self, Leds}, placa_arm::PlacaARM};

pub struct Operacion {}

impl Operacion {
    pub fn new() -> Self {
        Operacion {}
    }

    // Función que establece las banderas Z, N, C y V en la placa
    fn set_flags(&self, placa: &mut PlacaARM, resultado: i32) {
        placa.set_flag(2, resultado == 0); // Bandera Z
        placa.set_flag(3, resultado < 0);  // Bandera N
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
        // Verificar si se puede obtener el registro x
        if let Some(valor_x) = placa.get_register(x as usize) {
            // Obtener valor z dependiendo de si es inmediato
            let valor_z = if es_inmediato { z } else { placa.get_register(z as usize).unwrap_or(0) };
            let resultado = operacion(valor_x, valor_z);
            placa.set_register(des as usize, resultado); // Almacenar el resultado

            // Si se requiere establecer las banderas
            if bit_s {
                self.set_flags(placa, resultado); // Establecer banderas Z y N

                // Determinar el tipo de operación para establecer C y V
                let es_suma = std::ptr::eq(
                    operacion as *const (),
                    (|a, b| a + b) as fn(i32, i32) -> i32 as *const ()
                );
                let es_resta = std::ptr::eq(
                    operacion as *const (),
                    (|a, b| a - b) as fn(i32, i32) -> i32 as *const ()
                );

                // Manejar Carry (C) y Overflow (V)
                if es_suma {
                    let (_, carry) = (valor_x as u32).overflowing_add(valor_z as u32);
                    placa.set_flag(1, carry); // Bandera C
                    
                    // Determinar Overflow (V) para suma
                    let overflow = (valor_x > 0 && valor_z > 0 && resultado < 0) || 
                                   (valor_x < 0 && valor_z < 0 && resultado > 0);
                    placa.set_flag(0, overflow); // Bandera V
                } else if es_resta {
                    let (_, carry) = (valor_x as u32).overflowing_sub(valor_z as u32);
                    placa.set_flag(1, carry); // Bandera C
                    
                    // Determinar Overflow (V) para resta
                    let overflow = (valor_x < 0 && valor_z > 0 && resultado > 0) || 
                                   (valor_x > 0 && valor_z < 0 && resultado < 0);
                    placa.set_flag(0, overflow); // Bandera V
                } else {
                    // Para otras operaciones lógicas
                    placa.set_flag(1, false);  // Bandera C
                    placa.set_flag(0, false);  // Bandera V
                }
            }
        } else {
            println!("Error: índice fuera de rango o valor no encontrado para el índice {}", x);
        }
    }

    pub fn add(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32, es_inmediato: bool, bit_s: bool) {
        self.operar(placa, des, x, z, es_inmediato, bit_s, |a, b| a + b);
    }

    pub fn adc(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32) {
        // Verificar si se puede obtener el registro x
        if let Some(valor_x) = placa.get_register(x as usize) {
            // Obtener el valor z, usando el registro si no es inmediato
            let valor_z = placa.get_register(z as usize).unwrap_or(0);
            
            // Obtener el carry como un valor (0 o 1)
            let carry = placa.get_flag(2).unwrap_or(false) as i32; // Bandera C
            let resultado = valor_x + valor_z + carry; // Sumar x, z y el carry
            placa.set_register(des as usize, resultado); // Almacenar el resultado
            
            // Establecer las banderas
            self.set_flags(placa, resultado);

            // Manejar Carry (C) y Overflow (V)
            let (_, carry) = (valor_x as u32).overflowing_add(valor_z as u32);
            placa.set_flag(2, carry); // Bandera C

            // Determinar Overflow (V) para adc
            let overflow = (valor_x > 0 && valor_z > 0 && resultado < 0) || 
                        (valor_x < 0 && valor_z < 0 && resultado > 0);
            placa.set_flag(3, overflow); // Bandera V
        } else {
            println!("Error: índice fuera de rango o valor no encontrado para el índice {}", x);
        }
    }



    pub fn sub(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32, es_inmediato: bool, bit_s: bool) {
        self.operar(placa, des, x, z, es_inmediato, bit_s, |a, b| a - b);
    }

    pub fn sbc(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_register(x as usize), placa.get_register(z as usize)) {
            let carry = placa.get_flag(2).unwrap_or(false) as i32; // Obtener Carry
            let resultado = valor_x - valor_z - (1 - carry); // Calcular resultado con Carry
            placa.set_register(des as usize, resultado); // Almacenar el resultado
            self.set_flags(placa, resultado); // Establecer banderas
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

    pub fn rsc(&self, placa: &mut PlacaARM, rd: i32, rn: i32, operand2: i32) {
        // Obtener valores de los registros
        let valor_n = placa.get_register(rn as usize).unwrap_or(0); // Valor de Rn
        let carry = placa.get_flag(2).unwrap_or(false) as i32; // Bandera de carry (C)

        // Calcular resultado: Rn - Operand2 - (1 si C está activo, 0 si no)
        let resultado = valor_n - operand2 - carry;

        // Almacenar resultado en el registro de destino
        placa.set_register(rd as usize, resultado);

        // Establecer las banderas (Z y N)
        self.set_flags(placa, resultado);

        // Manejar Carry (C) y Overflow (V)
        let (underflow, carry_out) = (valor_n as u32).overflowing_sub(operand2 as u32 + carry as u32);
        placa.set_flag(2, carry_out); // Bandera C

        // Determinar Overflow (V)
        let overflow = (valor_n < 0 && operand2 > 0 && resultado > 0) || 
                    (valor_n > 0 && operand2 < 0 && resultado < 0);
        placa.set_flag(3, overflow); // Bandera V
    }

    pub fn teq(&self, placa: &mut PlacaARM, rd: i32, rn: i32, inmediato: bool) {
        let valor_rn = placa.get_register(rn as usize).unwrap_or(0);
        let valor_inmediato = if inmediato { 1 } else { 0 }; // Convertir bool a i32
        let resultado = valor_rn ^ valor_inmediato; // Operación TEQ es XOR

        // Establecer las banderas (Z y N)
        self.set_flags(placa, resultado);
    }

    pub fn cmn(&self, placa: &mut PlacaARM, rd: i32, rn: i32, inmediato: bool) {
        let valor_rn = placa.get_register(rn as usize).unwrap_or(0);
        let valor_inmediato = if inmediato { 1 } else { 0 }; // Convertir bool a i32
        let resultado = valor_rn + valor_inmediato; // Operación CMN es suma

        // Establecer las banderas (Z y N)
        self.set_flags(placa, resultado);

        // Manejar Carry (C) y Overflow (V)
        let (overflowing_sum, carry) = (valor_rn as u32).overflowing_add(valor_inmediato as u32);
        placa.set_flag(2, carry); // Bandera C

        // Determinar Overflow (V)
        let overflow = (valor_rn > 0 && valor_inmediato > 0 && resultado < 0) || 
                    (valor_rn < 0 && valor_inmediato < 0 && resultado > 0);
        placa.set_flag(3, overflow); // Bandera V
    }

    pub fn mvn(&self, placa: &mut PlacaARM, des: i32, z: i32, es_inmediato: bool) {
        let valor_z = if es_inmediato { z } else { placa.get_register(z as usize).unwrap_or(0) };
        placa.set_register(des as usize, !valor_z); // Almacenar el complemento
        self.set_flags(placa, !valor_z); // Establecer banderas
    }

    pub fn mov(&self, placa: &mut PlacaARM, des: i32, z: i32, es_inmediato: bool) {
        let valor_z = if es_inmediato { z } else { placa.get_register(z as usize).unwrap_or(0) };
        placa.set_register(des as usize, valor_z); // Almacenar el valor
        self.set_flags(placa, valor_z); // Establecer banderas
    }

    pub fn tst(&self, placa: &mut PlacaARM, x: i32, z: i32, es_inmediato: bool) {
        let valor_x = placa.get_register(x as usize).unwrap_or(0);
        let valor_z = if es_inmediato { z } else { placa.get_register(z as usize).unwrap_or(0) };
        let resultado = valor_x & valor_z; // Realizar AND para prueba
        self.set_flags(placa, resultado); // Establecer banderas
    }

    pub fn cmp(&self, placa: &mut PlacaARM, x: i32, z: i32, es_inmediato: bool) {
        let valor_x = placa.get_register(x as usize).unwrap_or(0);
        let valor_z = if es_inmediato { z } else { placa.get_register(z as usize).unwrap_or(0) };
        let resultado = valor_x - valor_z; // Calcular diferencia
        self.set_flags(placa, resultado); // Establecer banderas
        let carry = (valor_x as u32).overflowing_sub(valor_z as u32).1; // Calcular Carry
        placa.set_flag(2, carry); // Establecer bandera C
        // Calcular Overflow (V)
        let overflow = (valor_x < 0 && valor_z > 0 && resultado > 0) || (valor_x > 0 && valor_z < 0 && resultado < 0);
        placa.set_flag(3, overflow); // Establecer bandera V
    }

    pub fn rsb(&self, placa: &mut PlacaARM, des: i32, x: i32, z: i32) {
        if let (Some(valor_x), Some(valor_z)) = (placa.get_register(x as usize), placa.get_register(z as usize)) {
            let resultado = valor_z - valor_x; // Calcular RSB
            placa.set_register(des as usize, resultado); // Almacenar resultado
            self.set_flags(placa, resultado); // Establecer banderas
        } else {
            println!("Error: índices fuera de rango o valores no encontrados.");
        }
    }

    pub fn bmi(&self, placa: &mut PlacaARM, offset: i32) {
        if placa.get_flag(0).unwrap_or(false) { // Bandera N
            self.b(placa, offset); // Ajustar PC si es negativo
        }
    }

    pub fn instruccion_b(&self, placa: &mut PlacaARM, offset: i32) {
        self.b(placa, offset); // Realizar salto incondicional
    }

    pub fn bne(&self, placa: &mut PlacaARM, offset: i32) {
        if !placa.get_flag(1).unwrap_or(false) { // Bandera Z
            self.b(placa, offset); // Saltar si Z es cero
        }
    }

    pub fn blt(&self, placa: &mut PlacaARM, offset: i32) {
        if placa.get_flag(0).unwrap_or(false) != placa.get_flag(1).unwrap_or(false) { // Bandera N y Z
            self.b(placa, offset); // Saltar si N y Z son diferentes
        }
    }

    pub fn bgt(&self, placa: &mut PlacaARM, offset: i32) {
        if placa.get_flag(0).unwrap_or(false) == placa.get_flag(1).unwrap_or(false) && !placa.get_flag(1).unwrap_or(false) { // Bandera N y Z
            self.b(placa, offset); // Saltar si N y Z son iguales y Z es cero
        }
    }

    pub fn ble(&self, placa: &mut PlacaARM, offset: i32) {
        if placa.get_flag(1).unwrap_or(false) || (placa.get_flag(0).unwrap_or(false) != placa.get_flag(1).unwrap_or(false)) { // Bandera Z y N
            self.b(placa, offset); // Saltar si Z es cero o N y Z son diferentes
        }
    }

    pub fn bge(&self, placa: &mut PlacaARM, offset: i32) {
        if placa.get_flag(0).unwrap_or(false) == placa.get_flag(1).unwrap_or(false) { // Bandera N y Z
            self.b(placa, offset); // Saltar si N y Z son iguales
        }
    }

    pub fn beq(&self, placa: &mut PlacaARM, offset: i32) {
        if placa.get_flag(1).unwrap_or(false) { // Bandera Z
            self.b(placa, offset); // Saltar si Z es uno
        }
    }

        pub fn bhi(&self, placa: &mut PlacaARM, offset: i32) {
        if !placa.get_flag(1).unwrap_or(false) && !placa.get_flag(0).unwrap_or(false) {
            placa.set_register(15, placa.get_register(15).unwrap_or(0) + offset);
        }
    }

    pub fn bls(&self, placa: &mut PlacaARM, offset: i32) {
        if placa.get_flag(1).unwrap_or(false) || placa.get_flag(0).unwrap_or(false) {
            placa.set_register(15, placa.get_register(15).unwrap_or(0) + offset);
        }
    }

    pub fn swi(&mut self, placa: &mut PlacaARM) {
        // Lógica para manejar la interrupción de software
        // Por ejemplo, guardar el estado actual y saltar a un manejador de interrupciones
        println!("Interrupción de software ejecutada");
    }

    pub fn bl(&mut self, placa: &mut PlacaARM, offset: i32) {
        // Obtener el valor del PC actual, o usar un valor por defecto en caso de None
        let pc_actual = placa.get_register(15).unwrap_or(0);
        
        // Almacenar la dirección de retorno en LR (R14), apuntando a la siguiente instrucción
        let lr = pc_actual.wrapping_add(4);
        placa.set_register(14, lr);

        // Calcular y establecer la nueva dirección en PC (R15)
        let pc = pc_actual.wrapping_add(offset); // Aquí, operand2 ya es i32
        placa.set_register(15, offset); // Establecer el nuevo valor de PC

    }

    pub fn b(&self, placa: &mut PlacaARM, offset: i32) {
        if let Some(pc) = placa.get_register(15) {
            // Para ARM, PC apunta a la instrucción actual + 8 bytes
            let effective_pc = pc;// + 8;
            
            // El offset ya viene multiplicado por 4 del decodificador
            // Calculamos la nueva dirección sumando el PC efectivo y el offset
            let nueva_direccion = effective_pc + offset + 4;
            
            // Actualizamos el PC con la nueva dirección
            placa.set_register(15, nueva_direccion);
            
            //println!("Branch: PC={}, offset={}, nueva_direccion={}", pc, offset, nueva_direccion);
        } else {
            println!("Error: No se pudo obtener el valor del PC (R15)");
        }
    }


   pub fn ldr(&self, placa: &mut PlacaARM, rd: i32, rn: i32, operand2: i32, es_inmediato: bool, bit_s: bool) {
        // Calcula la dirección como `rn + operand2`
        let direccion = if es_inmediato {
            operand2 as usize
        } else {
            if let Some(valor_rn) = placa.get_register(rn.try_into().unwrap()) {
                valor_rn as usize + operand2 as usize
            } else {
                println!("Error: Registro R{} fuera de rango", rn);
                return;
            }
        };

        // Manejo especial para la dirección 0x800 (lectura de entradas desde teclado)
        if direccion == 0x800 {
            let valor_binario = self.simular_entradas_teclado(); // Simulando las entradas desde el teclado
            placa.set_register(rd.try_into().unwrap(), valor_binario);
            //println!("LDR R{}, [R{}, #0x800] -> Leyendo entradas: {:06b}", rd, rn, valor_binario);
        } else {
            // Leer del archivo dmem_io.dat
            let valor = self.leer_dmem_io(direccion / 4); // Dividimos por 4 para obtener la línea correcta
            match valor {
                Ok(v) => {
                    placa.set_register(rd.try_into().unwrap(), v);
                    //println!("LDR R{}, [R{}, #0x{:X}] -> Leyendo valor {} de dmem_io.dat", rd, rn, operand2, v);
                }
                Err(e) => {
                    println!("Error al leer dmem_io.dat: {}", e);
                }
            }
        }
    }

    fn simular_entradas_teclado(&self) -> i32 {
        // Simula las entradas de la placa leyendo los primeros 6 números del teclado
        println!("Simulando las 6 entradas (presiona 6 números):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        // Leer los primeros 6 números como un valor binario
        let mut valor_binario = 0;
        for (i, c) in input.trim().chars().take(6).enumerate() {
            if let Some(digit) = c.to_digit(10) {
                if digit > 1 {
                    println!("Error: Solo se permiten los números 0 o 1.");
                    return 0;
                }
                valor_binario |= (digit as i32) << (5 - i); // Construir el valor binario
            } else {
                println!("Error: Entrada no válida, se esperaba un número binario.");
                return 0;
            }
        }

        valor_binario
    }

    fn leer_dmem_io(&self, linea: usize) -> io::Result<i32> {
        let path = Path::new("src/utilitis/archivos/dmem_io.dat");
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            if index == linea {
                let line = line?;
                // Convertir de hexadecimal a decimal
                let valor = i32::from_str_radix(line.trim(), 16)
                    .unwrap_or_else(|_| {
                        println!("Error al convertir el valor hexadecimal: {}", line);
                        0
                    });
                return Ok(valor);
            }
        }
        
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Línea {} no encontrada en dmem_io.dat", linea)
        ))
    }

    pub fn str(&self, placa: &mut PlacaARM, rd: i32, rn: i32, operand2: i32, es_inmediato: bool, bit_s: bool) {
        // Calcula la dirección como rn + operand2
        let direccion = if es_inmediato {
            operand2 as usize // Si es inmediato, operand2 es directamente la dirección
        } else {
            match placa.get_register(rn.try_into().unwrap()) {
                Some(valor_rn) => valor_rn as usize + operand2 as usize,
                None => {
                    println!("Error: Registro R{} fuera de rango", rn);
                    return;
                }
            }
        };

        // Obtener el valor de rd y almacenarlo en la dirección de memoria
        match placa.get_register(rd.try_into().unwrap()) {
            Some(valor_rd) => {
                placa.set_memory(direccion, valor_rd);
                
                // Si la dirección es 0x800 (2048 en decimal), activar los LEDs
                if operand2 == 0x800 {
                    let mut leds = Leds::new();
                    leds.mostrar(valor_rd); // Mostrar el valor del registro en los LEDs
                }
                
                //println!("STR R{}, [R{}, #0x{:X}] -> Almacenando valor {} en dirección 0x{:X}", rd, rn, operand2, valor_rd, direccion);
            },
            None => println!("Error: Registro R{} fuera de rango", rd)
        }
    }
}
