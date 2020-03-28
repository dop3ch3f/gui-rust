extern crate gio;
extern crate gtk;
extern crate rand;
extern crate reqwest;
extern crate tempfile;
extern crate tokio;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Image, Label, Orientation};
use rand::Rng;
use reqwest::Response;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

async fn get_image_from_url(imageurl: &str) -> Result<String, Box<dyn Error>> {
    let mut response: Response = reqwest::get(imageurl).await?;
    let content_type = response.headers_mut().get("content-type").unwrap();
    let extension: Vec<&str> = content_type.to_str()?.split("/").collect();
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let filename = format!("{}.{}", n1, extension[1]);
    let mut out = File::create(&filename)?;
    let _ = out.write_all(&response.bytes().await?);
    Ok(filename)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let image_path = get_image_from_url("https://image.shutterstock.com/image-photo/portrait-surprised-cat-scottish-straight-260nw-499196506.jpg").await?;

    let app = Application::new(Some("com.dop3ch3f.rust-gui"), Default::default())
        .expect("failed to initialize gtk application");

    app.connect_startup(|app| {
        // bootstraping the application goes here
        // be careful so you dont have slow startups
        let window = ApplicationWindow::new(app);
        // register close event
        window.connect_delete_event(|win, _| {
            win.destroy();
            Inhibit(false)
        });
    });

    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);

        window.set_title("Cowsay");

        window.set_default_size(350, 70);

        let layout_box = GtkBox::new(Orientation::Vertical, 0);

        let label = Label::new(Some("Meow!\n   \\\n    \\"));

        layout_box.add(&label);

        let cat_image = Image::new_from_file(Path::new(&image_path));

        layout_box.add(&cat_image);

        let button = Button::new_with_label("Click me!");

        button.connect_clicked(|_| {
            println!("Clicked!");
        });

        layout_box.add(&button);

        window.add(&layout_box);

        window.show_all();
    });

    app.run(&env::args().collect::<Vec<_>>());

    Ok(())
}
