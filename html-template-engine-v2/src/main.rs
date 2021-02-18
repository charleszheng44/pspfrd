use std::fs; 
use serde_json;
use std::io::BufReader;
use std::io::prelude::*;
use html_template_engine::*;
use std::collections::HashMap;


struct Cli {
    html_template_path: std::path::PathBuf,
    context_path: std::path::PathBuf,
}

fn main() {
    let htp = std::env::args().
        nth(1).expect("the path to the html template is not given.");
    let cp = std::env::args().
        nth(2).expect("the path to the context json path is not given.");
    let cli = Cli {
        html_template_path: htp.into(),
        context_path: cp.into(),
    };
    
    // 1. read the context from the json path
    let context_js_str = fs::read_to_string(cli.context_path).
        expect("fail to read the context json file.");
    let context = serde_json::from_str::<HashMap<String, String>>(&context_js_str).
        expect("fail to serialize the context from the json string.");

    
    // 2. read the static html file
    let file = fs::File::open(cli.html_template_path).expect("");
    let file_buf = BufReader::new(file);
    for line in file_buf.lines() {
        let line_str = &line.expect("TODO");
        match get_content_type(line_str) {
            ContentType::Literal(content_str) => {
                println!("{}", content_str);
            },
            ContentType::Tag(TagType::ForTag) => {
                panic!("ForTag is not supported now");
            },
            ContentType::Tag(TagType::IfTag) => {
                panic!("IfTag is not supported now");
            },
            ContentType::TemplateVariable(mut expr_data) => {
                let expr_str = substitute_template_variable(&mut expr_data, &context).
                    expect("fail to substitute the template variable");
                println!("{}", expr_str);
            },
            ContentType::Unrecognized(err_str) => {
                panic!("fail to parse the html statement {}: {}", 
                    line_str, err_str);
            },
        } 
    }
}
