use std::collections::HashMap;

// File I/O
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Tera
use tera::Tera;
use tera::Context;
use tera::try_get_value;

// Yaml 
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;
use yaml_rust::Yaml;

// LOADING TEMPLATES -----------------------------------------------------

use lazy_static::lazy_static;

// Apparently we need serde
use serde_json::value::{to_value, Value};

// Using lazy_static! so that only what I'm actually using will be evaluated
// this is useful if I'm building just one page of the website
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

fn main() {
    
    // FILE INPUT -------------------------------------------------------
    // From rust by example
    
    // Create a path to the desired file
    let path = Path::new("posts/example1.md");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
   


    // Dividing the document into the yaml part and the markdown part
    let parts_iterator = content.split("BEGIN DOCUMENT");
    let parts: Vec<String> = parts_iterator.map(|s| s.to_string()).collect();


    // PARSING YAML ------------------------------------------------------
    let tags = &YamlLoader::load_from_str(&parts[0]).unwrap()[0];

    // Dump the YAML object
    println!("Yaml Object:");
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&tags).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);

    // Example getting something from the yaml
    // println!("Query the yaml: {}", tags["title"].as_str().unwrap());
   


    // PARSING HTML ----------------------------------------------------
    let html = markdown::to_html(&parts[1]);
    println!("HTML:\n{}", html);
   


    // INSERTING INTO TEMPLATE ------------------------------------- 

    // To render something in Tera we need two things
    // - name
    // - context
    // The prefix to the name wikk be automatically removed: /tempaltes/t.html -> t.html
    // The context can be either a data structure that implements the `Serialize` trait from
    // `serde_json` or an instance of `tera::Context`

    // EXAMPLE: Using the tera Context struct
    let mut context = Context::new();

    // Filling the context with data
    let product_name = String::from("Ventilatore di Zeb");
    context.insert("product", &product_name);
    let noise = 10000;
    context.insert("noise", &noise);
    // Assuming post.html exists in templates/ and has the context variables
    match TEMPLATES.render("post.html", &context) {
        Ok(s) => {
            // Printing the result 
            println!("{s}");
        },
        Err(why) => {
            println!("Problems in rendering from template: {}", why);
        }
    };
    
    /*
    // EXAMPLE Using a Serialize struct
    #[derive(Serialize)]
        struct Product {
        name: String
    }
    // or a struct
    tera.render("products/product.html", &Context::from_serialize(&product)?)?;
    */

}
