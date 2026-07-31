use justino_ui::css::{Color, CssParser, CssValue, DisplayValue, FlexDirection};
use justino_ui::UiError;

#[test]
fn test_css_parser_rules_and_specificity() -> Result<(), UiError> {
    let css_source = r#"
        /* Estilos principais */
        window {
            background-color: #f0f0f0;
            display: flex;
            flex-direction: column;
        }

        .btn-primary {
            background-color: #007bff;
            color: #ffffff;
            padding: 10px;
            border-radius: 4px;
        }

        #submit-btn {
            background-color: #28a745;
        }

        button:hover {
            opacity: 0.8;
        }

        text:lang(ar) {
            font-size: 18px;
        }
    "#;

    let mut parser = CssParser::new(css_source);
    let stylesheet = parser.parse()?;

    assert_eq!(stylesheet.rules.len(), 5);

    // Test specificity for #submit-btn vs .btn-primary vs button
    let computed = stylesheet.compute_style("button", Some("submit-btn"), &["btn-primary".to_string()], None, "pt-BR");
    
    let bg_color = computed.get("background-color").unwrap();
    assert_eq!(*bg_color, CssValue::Color(Color::rgb(40, 167, 69))); // #28a745 overwrites #007bff due to ID specificity

    Ok(())
}

#[test]
fn test_css_flexbox_properties_parsing() -> Result<(), UiError> {
    let css = r#"
        .flex-container {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
            gap: 15px;
        }
    "#;

    let mut parser = CssParser::new(css);
    let stylesheet = parser.parse()?;

    let computed = stylesheet.compute_style("div", None, &["flex-container".to_string()], None, "en-US");

    assert_eq!(computed.get("display"), Some(&CssValue::Display(DisplayValue::Flex)));
    assert_eq!(computed.get("flex-direction"), Some(&CssValue::Direction(FlexDirection::Row)));
    assert_eq!(computed.get("justify-content"), Some(&CssValue::Keyword("space-between".to_string())));
    assert_eq!(computed.get("align-items"), Some(&CssValue::Keyword("center".to_string())));
    assert_eq!(computed.get("gap"), Some(&CssValue::Px(15.0)));

    Ok(())
}
