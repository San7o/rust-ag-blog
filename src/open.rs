use std::path::Path;
use crate::PostData;
use std::error::Error;
use yaml_rust::YamlLoader;
use crate::generate::read_file_strings;
pub fn open_post(p: &Path) -> Result<PostData, Box<dyn Error>> {

    let content = read_file_strings(&p)?;
    
    // Dividing the document into the yaml part and the markdown part
    let parts_iterator = content.split("BEGIN DOCUMENT");
    let parts: Vec<String> = parts_iterator.map(|s| s.to_string()).collect();


    let tags = &YamlLoader::load_from_str(&parts[0])?[0]; // Parsing YAML
    // dump_yaml(tags); 

    return Ok(PostData {
        title: tags["title"].as_str().unwrap().to_owned(),
        subtitle: tags["subtitle"].as_str().unwrap().to_owned(),
        description: tags["description"].as_str().unwrap().to_owned(),
        image: tags["image"].as_str().unwrap().to_owned(),
        author: tags["author"].as_str().unwrap().to_owned(),
        text: parts[1].to_owned(),
        filename: tags["filename"].as_str().unwrap().to_owned(),
    });
}
