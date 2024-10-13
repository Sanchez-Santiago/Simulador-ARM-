// src/main.rs
mod utilitis; // Importar el módulo utilitis

fn main() -> std::io::Result<()> {
    // Crear una instancia de Archivo
    let archivo = utilitis::archivo::Archivo::nueva("./utilitis/archivo.txt");

    // Llamar al método de instancia lectura y almacenar el contenido
    let texto_archivo = archivo.lectura()?;

    // Crear una nueva instancia del menú
    let new_menu = utilitis::menu::Menu::new();

    // Llamar al método mostrar y pasar el contenido del archivo
    new_menu.mostrar(&texto_archivo)?;

    Ok(())
}
