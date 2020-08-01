extern crate gtk;
extern crate gio;
extern crate xlsxwriter;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Button};
use xlsxwriter::*;




fn xlsx_write_num_test(doc_path: &str) {
    let workbook = Workbook::new(doc_path);
    let num_format = workbook.add_format().set_num_format("##0.0");

    let mut sheet1 = workbook.add_worksheet(None).unwrap();
    sheet1.write_number(0, 0, 20.787, Some(&num_format)).unwrap();
    workbook.close().unwrap();
}



fn gtk_run() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        window.add(&button);

        window.show_all();
    });

    application.run(&[]);
}



fn main() {
    xlsx_write_num_test("before_gtk.xlsx");  // Correct "20,787" integer number in A1 Cell ("20.8" with formatting)
    gtk_run();
    xlsx_write_num_test("after_gtk.xlsx");  // Incorrect "20" integer number in A1 Cell ("20.0" with formatting)
}
