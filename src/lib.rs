//! ***Cuteness*** is a static site generator. It generates a simple [Go](https://go.dev/) web-server and builds the Markdown[^4] source files. It was created to offer extreme configuration, and that's mainly what we're going to talk about here.
//!
//! * [`cuteconfig.toml`](#cuteconfig)
//!     * [`[routing]`](#config.routing)
//!     * [`[misc]`](#config.misc)
//!     * [`[config]`](#config.config)
//! * [The front-matter](#frontmatter)
//!     * [Example](#frontmatter.example)
//! * [Templating](#templating)
//!     * [`{{page.*}}`](#templating.page)
//!         * [Example](#templating.page.example)
//!     * [`{{outer.*}}`](#templating.outer)
//!         * [Example](#templating.outer.example)
//! * [Source files](#sourcefiles)
//! * [Subcommands](#subcommands)
//!     * [`init`](#subcommands.init)
//!     * [`build`](#subcommands.build)
//!     * [`setup`](#subcommands.setup)
//!     * [`update`](#subcommands.update)
//!     * [`clean`](#subcommands.clean)
//!     * [`uninstall`](#subcommands.uninstall)
//!     * [`help`](#subcommands.help)
//! * [Styles](#styles)
//!     * [Using Sass](#styles.sass)
//!     * [Not using Sass](#styles.css)
//! * [Routing](#routing)
//! * [Preprocessors](#preprocessors)
//! 
//! ## `cuteconfig.toml` <a name="cuteconfig"></a>
//! 
//!
//! `cuteconfig.toml` is the file used to store configuration settings. It's default configuration in the current version is: ([latest version](https://github.com/blyxyas/cuteness/blob/main/cuteconfig.default.toml))
//!
//! ```toml
//! # cuteconfig.default.toml
//! [routing]
//! init_behaviour = "fmt.Printf(\"Starting webserver at port 8080\")"
//! fail_behaviour = "log.Fatal(err)"
//! imports = ["fmt", "log"]
//!
//! [misc]
//! latex = true # Add KaTeX support
//! html_lang = "en" # HTML Language
//! syntax_highlighting = true
//!
//! [config]
//! # Write here your custom templates!
//! ```
//!
//! ### `[routing]` <a name="config.routing"></a>
//!
//! This section handles code related to the generation of the Go web-server.
//! The web-server Handlebars template can be found [here](https://github.com/blyxyas/cuteness/blob/main/templates/routing.hbs)
//!
//! * `init_behaviour`: This code will run when the server is started.
//! * `fail_behaviour`: This code will run if the server encounters an error.
//! * `imports`: A list of modules to import for your custom code in `init_behaviour` and `fail_behaviour`.
//!
//! ### `[misc]` <a name="config.misc"></a>
//!
//! This section handles miscellaneous settings, usually related to preprocessors and very case-specific tools.
//!
//! * `latex`: Enables LaTeX[^1] equations.
//! * `html_lang`: Changes the starting `<html>` tag (e.g. *"es"* `<html lang="es">`).
//! * `syntax_highlighting`: Enables syntax highlighting using [`highlight.js`](https://highlightjs.org/).
//!
//! ### `[config]` <a name="config.config"></a>
//!
//! This section is used to store user-provided configurations. It can store any [TOML value](https://toml.io/en/v1.0.0#keyvalue-pair) (*strings, integers, arrays...*).
//!
//! ---
//!
//! All these sections can be used in your documents with `{{outer.*}}` (e.g. `{{outer.misc.html_lang}}`), we'll see more about templating in the next section.
//!
//! ## The front-matter <a name="frontmatter"></a>
//!
//! A front-matter is the initial heading before your Markdown contents. This heading contains some configuration options used to generate the webpage. Currently, the only mandatory field is `title`.
//!
//! * `title`: The current page's title.
//! * `pageconf` *(optional)*: User-provided page configuration (Key-value pairs).
//! * `additional_css` *(optional)*: Additional CSS files needed to properly render the page. **(`index.css` is imported by default)**
//!
//! ### Example <a name="frontmatter.example"></a>
//!
//! ```md
//! # my_file.md
//! ---
//! title: "My file"
//! ---
//!
//! Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc est, semper non
//! maximus et, mattis in leo. Nullam luctus, ligula id venenatis consequat, ligula
//! odio convallis eros, sed scelerisque tortor elit vitae massa. Aliquam efficitur
//! tempus purus sit amet eleifend. Mauris vel ex iaculis, iaculis nisl id, finibus
//! justo. Morbi gravida vel velit eget ultricies. In orci purus, porta ut nibh
//! blandit, vestibulum lobortis augue. Vestibulum venenatis finibus tellus, sit amet
//! venenatis elit rutrum ut. Donec posuere ipsum efficitur tortor viverra dignissim.
//! Donec urna libero, molestie id libero vitae, bibendum rutrum leo. Praesent
//! suscipit tincidunt ultrices. Sed finibus neque blandit velit venenatis volutpat.
//! In id dui sit amet quam ullamcorper viverra. In rutrum ante sapien, et tincidunt
//! ligula dignissim id. Curabitur hendrerit sagittis orci, in rhoncus dui venenatis
//! sit amet. Nunc sed enim arcu.
//! ```
//!
//! In this case, the front-matter only contains `title`, that being *"My file"*
//!
//! ## Templating <a name="templating"></a>
//!
//! ***Cuteness*** uses [`Handlebars-rs`](https://github.com/sunng87/handlebars-rust)[^3] and it exposes a templating API to the user with `page` and `outer`.
//!
//! ### `{{page.*}}` <a name="templating.page"></a>
//!
//! `{{page.*}}` is the interface that you can access in order to use page configuration. For example, you can use `{{page.title}}` to access the page's title.
//!
//! #### Example <a name="templating.page.example"></a>
//!
//! ```md
//! # my_file.md
//! ---
//! title: "My file"
//! pageconf: {
//!     software-version: "0.7.2",
//!     status: "beta"
//! }
//! ---
//!
//! Now I can use {{page.title}} to access this page's title.
//! Currently, our software is in version {{page.pageconf.software-version}}, that means that we're in a {{page.pageconf.status}} version.
//! ```
//!
//! This will get rendered to:
//!
//! ```html
//! <!-- [...] -->
//! <body>
//!     <div class="wrapper">
//!     <div class="cutesidebar">
//!         <ul>
//!
//!             <li><a href="introduction">Chapter 1: Introduction</a></li>
//!         </ul>
//!     </div>
//!     <div class="main-content">
//!         <p>Now I can use My file to access this page’s title.
//! Currently, our software is in version 0.7.2, that means that we’re in a beta version. </p>
//!     </div>
//! </div>
//! </body>
//! <!-- [...] -->
//! ```
//!
//! ### `{{outer.*}}` <a name="templating.outer"></a>
//!
//! `{{outer.*}}` is the interface to use if you want to access global settings found in `cuteconfig.toml`.
//!
//! #### Example <a name="templating.outer.example"></a>
//!
//! ```toml
//! # cuteconfig.toml
//! # [...]
//! [config]
//! authors = [
//!     "Alejandra González",
//!     "Alejandra's cat, Keepy 🐱"
//! ]
//! # [...]
//! ```
//!
//! ```hbs
//! # my_file.md
//! ---
//! title: "My file"
//! ---
//!
//! Now I can list the authors of the page:
//!
//! **Authors:**
//!
//! {{#each outer.config.authors}}
//!     - {{this}}
//!
//! {{/each}}
//! ```
//!
//! This will render to:
//!
//! ```html
//! <!-- [...] -->
//! <body>
//!     <div class="wrapper">
//!     <div class="cutesidebar">
//!         <ul>
//!
//!             <li><a href="introduction">Chapter 1: Introduction</a></li>
//!         </ul>
//!     </div>
//!     <div class="main-content">
//!         <p>Now I can list the authors of the page:</p>
//! <p><strong>Authors:</strong></p>
//! <p></p>
//! - Alejandra González
//! <p></p>
//! - Alejandra's cat, Keepy 🐱
//! <p></p>
//!
//!     </div>
//! </div>
//! </body>
//!
//! <!-- [...] -->
//!
//! ```
//!
//! ## Source files <a name="sourcefiles"></a>
//!
//! A normal file tree looks something like this:
//! ```text
//! .
//! ├── cuteconfig.toml
//! ├── src
//! │   └── introduction.md
//! │   └── [Other .md files]
//! └── SUMMARY.toml
//! ```
//!
//! All your Markdown files are located at the `src` directory; both `cuteconfig.toml` and `SUMMARY.toml` are located in the root directory. This is the default tree (generated by [`cuteness init`](#subcommands.init)) and it's the recommended way to start writing your contents.
//!
//! When creating a new file, you'll have to start the file writing a [front-matter](#frontmatter) and then the contents of your file. As explained in [*Templating*](#templating), you can use [Handlebars templates](https://handlebarsjs.com/).
//!
//! # Subcommands <a name="subcommands"></a>
//! ## `init` <a name="subcommands.init"></a>
//!
//! `cuteness init` is the command used to initialize a dummy directory, ready to be written. It will create:
//!
//! * `src` directory.
//! * `introduction.md` inside `src` with some [default contents](https://raw.githubusercontent.com/blyxyas/cuteness/main/introduction.default.md).
//! * `SUMMARY.toml` in the root directory with [default contents](https://github.com/blyxyas/cuteness/blob/main/SUMMARY.default.toml).
//! * `cuteconfig.toml` in the root directory with [default contents](https://github.com/blyxyas/cuteness/blob/main/cuteconfig.default.toml).
//!
//! You can start by writing on `introduction.md`, then executing `cuteness build`, executing `go run www/routing.go` and going to *http://localhost:8080/introduction*
//!
//! ## `build` <a name="subcommands.build"></a>
//!
//! `cuteness build` is used to build the project, it will create an output directory containing the built version (using all your configurations) of your `src` directory. If there are `.sass` files in the directory `src/styles` it will also compile those.
//!
//! ## `setup` <a name="subcommands.setup"></a>
//!
//! `cuteness setup` is a one-time command, it's used to get all necessary template files from the web. **It requires internet connection**. You can think of it as an enhanced `git clone` that only clones necessary files.
//!
//! **NOTE**: This command will create a directory called `cuteness-config` at your Cargo home (usually `~/.cargo/` on Unix systems) and store there all your internal configurations. (Do not edit manually.)
//!
//! ## `update` <a name="subcommands.update"></a>
//!
//! `cuteness update` will update the internal templates and styles to the latest version; you can think of it as an enhanced `git pull`.
//!
//! ## `clean` <a name="subcommands.clean"></a>
//!
//! `cuteness clean` will delete the output directory (default: `www`). It's not usually necessary.
//!
//! ## `uninstall` <a name="subcommands.uninstall"></a>
//!
//! `cuteness uninstall` will delete the internal templating and styling files stored in `<CARGO HOME>/cuteness-config/`.
//!
//! **This will not uninstall the binary**, you'll have to use `cargo uninstall cuteness`. This subcommand needs to be executed before doing that so it's a clean uninstall.
//! 
//! ## `help` <a name="subcommands.help"></a>
//! 
//! `cuteness help` displays a help message.
//! 
//! # Styles <a name="styles"></a>
//! 
//! Styling files are stored at `src/styles` and can be imported in a per-page basis using the `additional_css` optional key on the [page's front-matter](#frontmatter).
//! 
//! The special built-in `index.css` file is always imported, this file contains basic layout options for the correct display of your page. You can check the Sass source of `index.css` [here](https://github.com/blyxyas/cuteness/blob/main/src-styles/index.sass).
//! 
//! ## Using Sass <a name="styles.sass"></a>
//! 
//! You can use [Sass](https://sass-lang.com/) as a preprocessor for your files, just activate the feature `sass` when installing the binary (enabled by default) and store your `.sass` files in `src/styles` as any other `.css` file. They will be compiled with `cuteness build`.
//! 
//! ## Not using Sass <a name="styles.css"></a>
//! 
//! Almost the same, just locate your `.css` files at `src/styles` and they will not get compiled, but only copied to the output directory.
//! 
//! # Routing <a name="routing"></a>
//! 
//! When using `cuteness build`, an output directory containing some static files and a simple web-server will be generated.
//! 
//! An example of this webserver would be something like this:
//! 
//! ```go
//! package main
//! 
//! import (
//!     "fmt"
//!     "log"
//!     "net/http"
//!     "strings"
//! )
//! 
//! func main() {
//!     fmt.Printf("Starting webserver at port 8080")
//!     fs := http.FileServer(http.Dir("/home/alejandra/git/wawat/test/www/static"))
//!     http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
//!         if r.URL.Path != "/" && !strings.Contains(r.URL.Path, ".") {
//!             r.URL.Path += ".html"
//!         }
//!         fs.ServeHTTP(w, r)
//!     })
//! 
//!     if err := http.ListenAndServe(":8080", nil); err != nil {
//!         log.Fatal(err)
//!     }
//! }
//! ```
//! 
//! Note that **you need Go installed to run it[^5]**. The way to run it would be using:
//! 
//! ```bash
//! go run www/routing.go
//! ```
//!
//! This will create a local web-server which you can access by going to *http://localhost:8080/*
//! 
//! As the project is still in development, efforts about using actual servers available on the internet are still very far from being started.
//! 
//! # Preprocessors <a name="preprocessors"></a>
//! 
//! The files content are preprocessed before being written, these preprocessors are used to change \"straight quotes\" to “curly quotes”, or to change emojicodes "`:cat:`" to actual emojis 🐱. These preprocessors are applied automatically and should not cause any problems.
//! 
#![doc = ::document_features::document_features!()]
//! [^1]: The tool specifically uses [KaTeX](https://katex.org/), specialized on equations.
//!
//! [^3]: `Handlebars-rs` uses the [Handlebars templating language](https://handlebarsjs.com/)
//!
//! [^4]: Specifically, our parser ([`pulldown-cmark`](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/)) uses the [CommonMark](https://commonmark.org/) specification.
//! 
//! [^5]: There are some ideas about porting the generated web-server to Rust. As the project isn't v1.0 yet, this may change in the future.

use serde::{Deserialize, Serialize};
use core::panic;
use std::{
    borrow::Cow,
    fs::{create_dir, remove_dir_all},
    path::{Path, PathBuf},
    process::Command,
};

use emojis::get_by_shortcode;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REQUOTE: Regex = Regex::new("\"(.*?)\"").unwrap();
    static ref REEMOJI: Regex = Regex::new(":(.*?):").unwrap();
    pub static ref CONFIG_PATH: PathBuf = home::cargo_home()
        .expect("Couldn't get Cargo home")
        .join("cuteness-config");
}

const REPO_URL: &str = "https://github.com/blyxyas/cuteness.git";

/// Replace straight quotes (") with curly quotes, U+201C (“) and U+201D (”)
#[inline(always)]
pub fn curly_quotes(content: &str) -> Cow<'_, str> {
    REQUOTE.replace_all(content, "“$1”")
}

/// Replaces all emojicodes (:cat:) to real emojis
#[inline(always)]
pub fn emojis(content: &str) -> String {
    let mut result = <&str>::clone(&content).to_string();
    for cap in REEMOJI.find_iter(content) {
        if let Some(emoji) = get_by_shortcode(&content[cap.start() + 1..cap.end() - 1]) {
            result = content.replace(cap.as_str(), emoji.as_str());
        };
    }
    result
}

#[inline]
pub fn setup() {
    if CONFIG_PATH.exists() {
        return;
    }

    create_dir(CONFIG_PATH.as_path())
        .unwrap_or_else(|e| panic!("Couldn't create directory `{}`: {e}", CONFIG_PATH.display()));

    // Initialize git repo

    Command::new("git")
        .current_dir(CONFIG_PATH.as_path())
        .args(["init"])
        .status()
        .expect("Couldn't initialize repository");

    // Add this repo as remote

    Command::new("git")
        .current_dir(CONFIG_PATH.as_path())
        .args(["remote", "add", "origin", REPO_URL])
        .status()
        .expect("Couldn't add remote");

    // Enable sparse checkout

    Command::new("git")
        .current_dir(CONFIG_PATH.as_path())
        .args(["config", "core.sparseCheckout", "true"])
        .status()
        .expect("Couldn't enable `core.sparseCheckout`");

    // Add templates to sparse-checkout

    std::fs::write(
        CONFIG_PATH
            .join(".git")
            .join("info")
            .join("sparse-checkout"),
        "templates/*",
    )
    .unwrap_or_else(|e| {
        panic!(
            "Couldn't write to {}: {e}",
            CONFIG_PATH
                .join(".git")
                .join("info")
                .join("sparse-checkout")
                .display()
        )
    });

    Command::new("echo")
        .current_dir(CONFIG_PATH.as_path())
        .args(["templates/*", ">>", ".git/info/sparse-checkout"])
        .status()
        .expect("Couldn't add templates to `.git/info/sparse-checkout`");

    // Pull

    Command::new("git")
        .current_dir(CONFIG_PATH.as_path())
        .args(["pull", "--depth=1", "origin", "main"])
        .status()
        .expect("Couldn't pull");

    println!("Cuteness was successfully configured!");
}

#[inline]
pub fn check_for_updates() {
    Command::new("git")
        .current_dir(CONFIG_PATH.as_path())
        .args(["pull", "--depth=1", "origin", "main", "--rebase"])
        .status()
        .expect("Couldn't pull");

    println!("Repository updated!");
}

#[inline]
pub fn uninstall() {
    let config_path = CONFIG_PATH.as_path();
    if config_path.exists() {
        remove_dir_all(config_path).unwrap_or_else(|e| {
            panic!(
                "Couldn't remove directory {}: {e}",
                <&Path>::clone(&config_path).display()
            )
        });
    }
}

pub fn init() {
    create_dir("src").unwrap_or_else(|e| panic!("Couldn't create directory 'src': {e}"));

    std::fs::write(
        "cuteconfig.toml",
        include_bytes!("../cuteconfig.default.toml"),
    )
    .unwrap_or_else(|e| panic!("Couldn't create `cuteconfig.toml`: {e}"));

    std::fs::write("SUMMARY.toml", include_bytes!("../SUMMARY.default.toml"))
        .unwrap_or_else(|e| panic!("Couldn't create `SUMMARY.md`: {e}"));

    std::fs::write(
        "src/introduction.md",
        include_bytes!("../introduction.default.md"),
    )
    .unwrap_or_else(|e| panic!("Couldn't create `src/introduction.md`: {e}"));
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Method {
	POST,
	GET,
}

impl Method {
	pub fn method(&self) -> &str {
		if *self == Method::GET {
			return "GET"
		};
		"POST"
	}
}