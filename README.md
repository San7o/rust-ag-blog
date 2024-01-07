# Markdown blog in Rust

Rust toolchain for managing markdown posts for a static blog, hosted on github pages. For each markdown blog post, the toolchain will parse it to html and yaml, a new post page will be created from an html template made in wordpress.

### What I used:
- [Markdown](https://github.com/wooorm/markdown-rs) to parse the markdown files into html
- [Tera](https://github.com/Keats/tera) template engine to insert html to a template file
- [yaml-rust](https://github.com/chyh1990/yaml-rust) to parse Yaml data
- Local installation with [WordPress](https://github.com/WordPress/WordPress) to generate the templates
  - Thanks to used [Elementor](https://it.wordpress.org/plugins/elementor/) editor and [Export WP page to html](https://wordpress.org/plugins/export-wp-page-to-static-html/).

### Structure:
- posts/          .md post files
- site/           the main site 
  - blog/         the main blog page
  - post/         all the posts' html 
  - index.html    the main site index 
- src/            the source code
- templates/      templates folder    
  - blog.html     custom template for the blog page  
  - post.html     custom template for a post
  
### Must have jaml tags 
In every post file there `must` exist a yaml tag for each of the following:
- filename: Short string without spaces, the html file will be named after this
- title: The main title displayed
- subtitle: Will be displayed under the title, leave empty ("") for no subtitle 
- description: A summary of the blog content, will be displayed in the blog page
- author: Author name
- "BEGIN DOCUMENT" keyword: to divide tags section from the blog content section

### Recommendations
Use .jpg images or any compressed format (not .png)

Features / TODO
- [x] Markdown to Html 
- [x] Html to Template 
- [x] Support for yaml tags
- [x] Decent templates
- [x] Link MD to templates
- [x] Post page 
- [ ] Blog Page
- [ ] Images from png to jpg
