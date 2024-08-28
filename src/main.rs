use std::{env, process};
use std::path::PathBuf;
use askama::Template;
use clap::Parser;
use url::Url;
use crate::resources::{rebuild_cache, resource_url};
use crate::utils::StringExt;

mod resources;
mod utils;

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")]
struct IndexTemplate {
    stylesheet: Url,
    dictionary_js: Url,
    words_output: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    words: Vec<String>,

    #[clap(short, long)]
    english_to_latin: bool,

    #[clap(long)]
    rebuild_cache: bool,

    #[clap(short, long)]
    program_dir: Option<PathBuf>,
}

fn main() {
    let args = CliArgs::parse();

    if args.rebuild_cache {
        rebuild_cache();
    }

    let words_dir = args.program_dir
        .or_else(|| env::var_os("WORDS_DIR").map(PathBuf::from))
        .unwrap_or_else(|| {
            env::current_exe().unwrap().parent().unwrap().to_path_buf()
        });

    let words_exe = words_dir.join("words");

    let mut word_args = Vec::with_capacity(args.words.len() + 1);
    if args.english_to_latin {
        word_args.push("~E".to_string());
    }
    word_args.extend(args.words);

    let output = process::Command::new(&words_exe)
        .current_dir(&words_dir)
        .args(&word_args)
        .output()
        .expect("Failed to execute words program");

    let words_output = String::from_utf8(output.stdout)
        .expect("Words output contains illegal characters")
        .escape_template_literal()
        ;

    let index_template = IndexTemplate {
        stylesheet: resource_url("assets/dictionary.css"),
        dictionary_js: resource_url("assets/dictionary.js"),
        words_output,
    };

    let rendered = index_template.render().expect("Failed to render template");
    println!("{}", rendered);
}
