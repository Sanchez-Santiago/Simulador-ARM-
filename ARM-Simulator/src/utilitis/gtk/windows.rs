use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Orientation, ScrolledWindow, TextView};

fn on_activate(application: &Application) {
    // Crear la ventana principal
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Consola de Salida")
        .default_width(400)
        .default_height(300)
        .build();

    // Crear una caja vertical para organizar los widgets
    let vbox = GtkBox::new(Orientation::Vertical, 5);

    // Crear el área de texto para mostrar la salida de consola
    let text_view = TextView::new();
    text_view.set_editable(false);

    // Envolver el `TextView` en un `ScrolledWindow` para agregar una barra de desplazamiento
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_vexpand(true);
    scrolled_window.set_child(Some(&text_view));

    // Crear el primer botón
    let button1 = Button::with_label("Botón 1");

    // Crear el segundo botón
    let button2 = Button::with_label("Botón 2");

    // Agregar los botones y el área de texto a la caja vertical
    vbox.append(&button1);
    vbox.append(&button2);
    vbox.append(&scrolled_window);

    // Establecer la caja en la ventana principal
    window.set_child(Some(&vbox));
    window.present(); // Presentar la ventana
}
