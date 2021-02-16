use html_template_engine::{ExpressionData, ContentType, TagType};
use html_template_engine as hte;

#[test]
fn check_template_var_test() {
    let content = ExpressionData {
        head: Some("Hi ".to_string()),
        variable: "name".to_string(),
        tail: Some(" bye".to_string()),
    };
    assert_eq!(
        ContentType::TemplateVariable(content),
        hte::get_content_type("Hi {{name}} bye")
    );
}
#[test]
fn check_for_tag_test() {
    assert_eq!(
        ContentType::Tag(TagType::ForTag),
        hte::get_content_type("{% for name in names %} bye")
    );
}
#[test]
fn check_if_tag_test() {
    assert_eq!(
        ContentType::Tag(TagType::IfTag),
        hte::get_content_type("{% if name == 'Bob' %}")
    );
}
#[test]
fn check_literal_test() {
    let s = "<h1>Hello world</h1>";
    assert_eq!(ContentType::Literal(s.to_string()), hte::get_content_type(s));
}

#[test]
fn check_get_expression_data_test() {
    let expression_data = ExpressionData {
        head: Some("Hi ".to_string()),
        variable: "name".to_string(),
        tail: Some(",welcome".to_string()),
    };

    assert_eq!(Ok(expression_data), hte::get_expression_data("Hi {{name}},welcome"));
}
