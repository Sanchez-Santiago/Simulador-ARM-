// src/main.rs
mod utilitis; // Importar el módulo utilitis

fn main() -> std::io::Result<()> {
    // Crear una nueva instancia del menú
    let new_menu = utilitis::menu::Menu::new();
    // Llamar al método mostrar y pasar el contenido del archivo
    new_menu.mostrar();

    Ok(())
}