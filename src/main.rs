#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use crate::egui::Layout;
use crate::egui::Button;
use crate::egui::Resize;
use crate::egui::TextEdit;
use crate::egui::RichText;
use crate::add::add_post;
use crate::open::open_post;

use crate::generate::generate_page;
use crate::add::PostData;

use rfd::FileDialog;

use std::process;

mod generate;
mod add;
mod open;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 540.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Generatore Sito",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

#[derive(Default)]
enum State {
    Add,
    Remove,
    Res,
    #[default]
    None
}

#[derive(Default)]
struct MyApp {
    post: PostData,
    state: State,
    result: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            // Heading text
            ui.heading("Generatore Sito");

            // Blank space 
            ui.label("");
           
            // Left Panel
            egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {

                ui.columns(1, |ui| {

                    let post_button = Button::new("Aggiungi Post");
                    let exit_button = Button::new("Esci");

                    // Buttons
                    if ui[0].add(post_button).clicked() {
                        self.result = "".to_owned();
                        self.post = PostData::default();
                        self.state = State::Add;
                    }
                    if ui[0].button("Modifica Post").clicked() {
                     let file = FileDialog::new()
                        .add_filter("md", &["md"])
                        .set_directory("./posts")
                        .pick_file();
                        
                     match &file {
                        Some(f) => {
                            match &open_post(f.as_path()) {
                                Ok(p) => {
                                    self.post = p.clone();
                                    self.state = State::Add;
                                },
                                Err(why) => {
                                    self.result = format!("Errore nella selezione del file: {}", why);
                                    self.state = State::Res;
                                }
                            };
                        },
                        None => {
                            self.result = String::from("Errore nella selezione del file");
                            self.state = State::Res;
                        }
                     };
                    }
                    if ui[0].button("Elimina Post").clicked() {
                        self.state = State::Remove;
                    }
                    if ui[0].button("Genera Sito").clicked() {
                        match generate_page() {
                            Ok(()) => {
                                self.result ="Sito generato correttamente".to_owned();
                            },
                            Err(why) => {
                                self.result = format!("Ci sono stati degli errori nella generazione del sito: {}", why);
                            }
                        }
                        self.state = State::Res;
                    }
                    if ui[0].add(exit_button).clicked() {
                    
                        process::exit(0);
                    }


                });

           });

            match self.state {
                State::Add => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                           
                    ui.horizontal(|ui|{
                        if ui.button("Salva").clicked() {
                            match add_post(&self.post) {
                                Ok(()) => {
                                    self.result ="Salvataggio avvenuto correttamente".to_owned();
                                },
                                Err(why) => {
                                    self.result = format!("Ci sono stati degli errori nel salvataggio: {}", why);
                                }
                            }
                            // self.state = State::Res;
                        }
                        if ui.button("Cancella").clicked() {
                            self.post = PostData::default();
                        }

                        ui.label(RichText::new(&self.result));
                    });

                        ui.columns(2, |ui| {
                            ui[0].label("Titolo");
                            ui[0].text_edit_singleline(&mut self.post.title);
                        
                            ui[1].label("Sottotitolo");
                            ui[1].text_edit_singleline(&mut self.post.subtitle);
                        
                            ui[0].label("Descrizione");
                            ui[0].text_edit_singleline(&mut self.post.description);
                            
                            ui[1].label("Immagine");
                            ui[1].text_edit_singleline(&mut self.post.image);
                            
                            ui[0].label("Autore");
                            ui[0].text_edit_singleline(&mut self.post.author);

                            ui[1].label("Nome del file");
                            ui[1].text_edit_singleline(&mut self.post.filename);

                        });
                    ui.label("Testo");
                        
                        let text_field = TextEdit::multiline(&mut self.post.text).desired_width(f32::INFINITY);
                        ui.add(text_field);

                    });
        
                },
                State::Remove => {

                },
                State::Res => {
                    ui.label(RichText::new(self.result.clone()));
                },
                State::None => {},
            }
       });
    }
}
