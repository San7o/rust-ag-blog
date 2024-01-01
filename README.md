# Markdown blog in Rust

Rust toolchain for managing markdown posts for a static blog, hosted on github pages. For each markdown blog post, the toolchain will parse it to html and yaml, a new post page will be created from an html template made in wordpress.

I used:
- [Markdown](https://github.com/wooorm/markdown-rs) to parse the markdown files into html
- [Tera](https://github.com/Keats/tera) template engine to insert html to a template file
- [yaml-rust](https://github.com/chyh1990/yaml-rust) to parse Yaml data
- Local installation with wordpress to generate the template, I used elementor editor.

Features
- [x] Markdown to Html 
- [x] Html to Template 
- [x] Support for yaml tags
- [x] Decent templates
- [ ] Link MD to templates
