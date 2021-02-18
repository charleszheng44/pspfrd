use std::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData),
    Tag(TagType),
    Unrecognized(String),
}

#[derive(Debug, PartialEq)]
pub enum TagType {
    ForTag,
    IfTag,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionData {
    pub expression: String,
    pub variables: Vec<String>,
    pub gen_html: String,
}

impl fmt::Display for ExpressionData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExpressionData{{ \
        expression: {},\
        variables: {:?},\
        gen_html: {},\
        }}", self.expression, self.variables, self.gen_html)
    }
}

pub fn get_content_type(input: &str) -> ContentType {
    if let Some(start) = input.find("{%") {
        if let Some(end) = input.find("%}") {
            if start > end || start + 1 == end {
                return ContentType::Unrecognized(
                    "the left parenthesis must appear before the right parenthesis".into());
            }
            let tag_clause = input[start+2..end].to_string();
            if tag_clause.contains("if") {
                return ContentType::Tag(TagType::IfTag);
            }

            if tag_clause.contains("for") {
                return ContentType::Tag(TagType::ForTag);
            }
            return ContentType::Unrecognized(
                "tag content is not for or if".into());
        } 
        return ContentType::Unrecognized(
            "the right parenthesis does not exist".into());
    }

    if input.contains("{{") {
        // 2. TemplateVariable
        if let Some(expr_data) = get_expression_data(input){
             return ContentType::TemplateVariable(expr_data);
        }
    } 
    
    return ContentType::Literal(input.into());
}

pub fn get_expression_data(inp: &str) -> Option<ExpressionData> {
    let expr = String::from(inp);
    let str_lst_iter = inp.split_whitespace();
    let mut vars = vec!{};
    for word in str_lst_iter {
        if word.starts_with("{{") && word.ends_with("}}") {
            vars.push(word.to_string());
        }
    }
    
    if vars.len() == 0 {
        return None;
    }

    Some(ExpressionData{
        expression: expr,
        variables: vars,
        gen_html: "".into(),
    })
}

pub fn substitute_template_variable(
    expr_data: &mut ExpressionData, 
    context: &HashMap<String, String>) -> Result<String, String>{
    expr_data.gen_html = expr_data.expression.clone();
    for var in &expr_data.variables {
        let start = match var.find("{") {
            Some(i) => i,
            None => return Err("{{ is not found".into()),
        };
        let end = match var.find("}") {
            Some(i) => i,
            None => return Err("}} is not found".into()),
        };
        let var_without_brace = &var[start+2..end];
        let val = match context.get(var_without_brace) {
            Some(v) => v,
            None => return Err(
                format!("the corresponding entry(key={}) is not found", 
                    var_without_brace)),
        };
        expr_data.gen_html = expr_data.gen_html.replace(var, val);
    }
    Ok(expr_data.gen_html.clone())
}
