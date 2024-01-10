use tera::Context;
use std::fs::File;
use crate::generate::TEMPLATES;
use std::io::Write;
use std::error::Error;

#[derive(Default, Clone)]
pub struct PostData {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub image: String,
    pub author: String,
    pub filename: String,
    pub text: String,
}


pub fn add_post(post: &PostData) -> Result<(), Box<dyn Error>> {

    // Checks
    if post.title == "" {
        return Err(String::from("Nessun titolo inserito").into());
    }

    if post.subtitle == "" {
        return Err(String::from("Nessun sottotitolo inserito").into());
    }

    if post.description == "" {
        return Err(String::from("Nessuna descrizione inserita").into());
    }

    // TODO
    // if (post.image)
    
    if post.filename == "" {
        return Err(String::from("Nessun nome del file inserito").into());
    }
    if post.filename.contains(" ") {
        return Err(String::from("Il nome del file non può contenere spazi").into());
    }


    // INSERTING INTO TEMPLATE 
    // To render something in Tera we need two things
    // - name
    // - context
    // The prefix to the name wikk be automatically removed: /tempaltes/t.html -> t.html
    // The context can be either a data structure that implements the `Serialize` trait from
    // `serde_json` or an instance of `tera::Context`

    // EXAMPLE: Using the tera Context struct
    let mut context = Context::new();
    

    // Filling the context
    context.insert("filename", &post.filename);
    context.insert("title", &post.title);
    context.insert("subtitle", &post.subtitle);
    context.insert("description", &post.description);
    context.insert("author", &post.author);
    context.insert("text", &post.text);
    context.insert("image", &post.image);

    // dump_final(context); 
    
    // WRITE TO FILE
    let mut file = File::create(
                    format!("./posts/{}.md", &post.filename)
                    )?;
    let s = TEMPLATES.render("md_post.md", &context)?;
    
    // Printing the result 
     file.write_all(&s.into_bytes())?;


    Ok(())
}
