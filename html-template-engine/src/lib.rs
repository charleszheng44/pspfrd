use std::fmt;

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
    pub head: Option<String>,
    pub variable: String,
    pub tail: Option<String>,
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
        return match get_expression_data(input){
            Ok(expr_data) => ContentType::TemplateVariable(expr_data),
            Err(e) => ContentType::Unrecognized(e.to_string()),
        }
    } 
    
    return ContentType::Literal(input.into());
}


#[derive(Debug, PartialEq)]
pub struct GetExpressionError(String, String);

impl fmt::Display for GetExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fail to get expression data from input {}: {}", self.0, self.1)
    }
} 

macro_rules! unwrap_or_return_err {
    ($opt: expr, $err: expr) => {
        match $opt {
            Some(val) => val,
            None => return Err($err),
        }
    }
}

pub fn get_expression_data(inp: &str) -> Result<ExpressionData, GetExpressionError> {
    let start = unwrap_or_return_err!(inp.find("{{"), 
        GetExpressionError(
            inp.into(), 
            "can't parse it into an ExpressionData as there exist no {{".into()));
    let end = unwrap_or_return_err!(inp.find("}}"), 
        GetExpressionError(
            inp.into(), 
            "can't parse it into an ExpressionData as there exist no {{".into()));
    
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
        tail = Some(inp[end+2..inp.len()].into());
    }
    
    Ok(ExpressionData{
        head,
        variable: inp[start+2..end].into(),
        tail,
    })
}
