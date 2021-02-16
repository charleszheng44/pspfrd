#![allow(dead_code)]

use std::fmt;

enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData),
    Tag(TagType),
    Unrecognized(String),
}

enum TagType {
    ForTag,
    IfTag,
}

struct ExpressionData {
    head: Option<String>,
    variable: String,
    tail: Option<String>,
}

impl fmt::Display for ExpressionData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExpressionData{{")?;
        if let Some(head_str) = &self.head {
            write!(f, "head:{},", head_str)?;
        } else {
            write!(f, "head:None")?;
        }
        write!(f, "variable:{}", self.variable)?;
        if let Some(tail_str) = &self.tail {
            write!(f, "tail:{}", tail_str)?;
        } else {
            write!(f, "tail:None")?;
        }
        write!(f, "}}")
    }
}

fn get_content_type(input: &str) -> ContentType {
    if input.starts_with("{%") {
        // 1. Tag 
        if !input.ends_with("%}") {
            return ContentType::Unrecognized("TODO".into());
        }

        // 1.1 ForTag 
        if input.starts_with("{% for ") || input == "{% endfor %}"{
            return ContentType::Tag(TagType::ForTag);
        }

        // 1.2 IfTag
        if input.starts_with("{% if ") || input == "{% endif %}" {
            return ContentType::Tag(TagType::IfTag);
        }

        // 1.3 Unrecognized
        return ContentType::Unrecognized(
            format!("invalid tag statement {}", input));
    } 

    if input.contains("{{") {
        // 2. TemplateVariable
        return match get_expression_data(input){
            Ok(expr_data) => ContentType::TemplateVariable(expr_data),
            Err(e) => ContentType::Unrecognized(e.to_string()),
        }
    } 
    
    return ContentType::Literal(input.into());
}

struct GetExpressionError(String, String);

impl fmt::Display for GetExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fail to get expression data from input {}: {}", self.0, self.1)
    }
} 

fn get_expression_data(inp: &str) -> Result<ExpressionData, GetExpressionError> {
    let start = inp.find("{{").unwrap();
    let end = inp.find("}}").unwrap();
    
    if start >= end {
        return Err(GetExpressionError(inp.into(), 
                "the left parenthesis appears after right parenthesis".into()));
    }

    if start + 1 == end {
        return Err(GetExpressionError(inp.into(), 
                "the left parenthesis overlaps with the right parenthesis".into()));
    }

    let mut head: Option<String> = None;
    let mut tail: Option<String> = None;
    
    if start != 0 {
        head = Some(inp[0..start].into());
    }

    if end != inp.len() {
        tail = Some(inp[end..inp.len()].into());
    }
    
    Ok(ExpressionData{
        head,
        variable: inp[start+2..end].into(),
        tail,
    })
}
