use std::collections::HashMap;

// File I/O
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// Tera
use tera::Tera;
use tera::Context;
use tera::try_get_value;

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
        //tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

/*
// Honestly I have no idea of waht is this and shy this is needed
pub fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(s).unwrap())
}
*/

fn main() {
    
    // FILE INPUT -------------------------------------------------------
    // From rust by example
    
    /*
    // Create a path to the desired file
    let path = Path::new("posts/example1.md");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
                // Using markdown library to parse the file
        Ok(_) => println!("{}", markdown::to_html(&s)),
    }
    */
    
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
