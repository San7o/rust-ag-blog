// Std
use std::collections::HashMap;
use std::error::Error;

// File I/O
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs;

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
        // Disabled autoescaping to display actual html
        tera.autoescape_on(vec![".sql"]);
        tera
    };
}

#[derive(Debug)]
struct PostPreview {
    title: String,
    description: String,
    image: String,
    filename: String,
}

use serde::Serializer;
use serde::Serialize;
use serde::ser::SerializeStruct;

impl Serialize for PostPreview {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PostPreview", 4)?;

        state.serialize_field("title", &self.title)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("image", &self.image)?;
        state.serialize_field("filename", &self.filename)?;

        state.end()
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    
    let md_posts = fs::read_dir("posts/")?;
        
    let mut posts_vector: Vec<PostPreview> = Vec::new(); 

    for md_post in md_posts {
        
        let path = &(md_post?).path();
        println!("Working on: {}", path.display());

        let content = read_file_strings(&path)?;
        
        // Dividing the document into the yaml part and the markdown part
        let parts_iterator = content.split("BEGIN DOCUMENT");
        let parts: Vec<String> = parts_iterator.map(|s| s.to_string()).collect();


        let tags = &YamlLoader::load_from_str(&parts[0])?[0]; // Parsing YAML
        // dump_yaml(tags); 


        let html = markdown::to_html(&parts[1]); // Parsing HTML
        // println!("HTML:\n{}", html); // Dump html


        // INSERTING INTO TEMPLATE 
        // To render something in Tera we need two things
        // - name
        // - context
        // The prefix to the name wikk be automatically removed: /tempaltes/t.html -> t.html
        // The context can be either a data structure that implements the `Serialize` trait from
        // `serde_json` or an instance of `tera::Context`

        // EXAMPLE: Using the tera Context struct
        let mut context = Context::new();
        
        let file_name = tags["filename"].as_str().unwrap();

        // Filling the context
        context.insert("title", tags["title"].as_str().unwrap());
        context.insert("subtitle", tags["subtitle"].as_str().unwrap());
        context.insert("author", tags["author"].as_str().unwrap());
        context.insert("text", &html);
    
        // dump_final(context); 
        
        // WRITE TO FILE
        let mut file = File::create(
                         format!("./site/post/index.php/{}.html", file_name)
                        )?;
        match TEMPLATES.render("post.html", &context) {
            Ok(s) => {
                // Printing the result 
                file.write_all(&s.into_bytes())?;
            },
            Err(why) => {
                println!("Problems in rendering from template: {}", why);
            }
        };


        // Update the posts vector for blog page 
        posts_vector.push(PostPreview {
            title: tags["title"].clone().into_string().unwrap(),
            description: tags["description"].clone().into_string().unwrap(),
            image: tags["image"].clone().into_string().unwrap(),
            filename: tags["filename"].clone().into_string().unwrap(),
        });

    } 

    // Create the blog page
    let mut context = Context::new();
    context.insert("posts", &posts_vector);
    let mut file = File::create("./site/blog/index.php/blog.html")?;
    
    // Test for now
    match TEMPLATES.render("blog.html", &context) {
        Ok(s) => {
            // Printing the result 
            file.write_all(&s.into_bytes())?;
        },
        Err(why) => {
            println!("Problems in rendering from template: {}", why);
        }
    };



    Ok(())
}

fn read_file_strings(path: &Path) -> Result<String, Box<dyn Error>> {

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = File::open(&path)?;

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut content = String::new();
    let _ = file.read_to_string(&mut content)?;

    Ok(content)
   
}

fn dump_yaml(tags: &Yaml) {

    // Example getting something from the yaml
    // println!("Query the yaml: {}", tags["title"].as_str().unwrap());
 

    // Dump the YAML object
    println!("Yaml Object:");
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&tags).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);

}

fn dump_final(context: Context) {

    // Printing html
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
