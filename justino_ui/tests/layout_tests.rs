use justino_ui::css::CssParser;
use justino_ui::i18n::Locale;
use justino_ui::layout::{FlexboxEngine, UiNode};
use justino_ui::UiError;

#[test]
fn test_flexbox_layout_computation() -> Result<(), UiError> {
    let css = r#"
        .container {
            display: flex;
            flex-direction: row;
            width: 500px;
            height: 100px;
            gap: 10px;
        }

        .item {
            width: 100px;
            height: 50px;
        }
    "#;

    let mut parser = CssParser::new(css);
    let stylesheet = parser.parse()?;
    let locale = Locale::pt_br();

    let mut root = UiNode::element(1, "div", vec!["container".to_string()]);
    let item1 = UiNode::element(2, "div", vec!["item".to_string()]);
    let item2 = UiNode::element(3, "div", vec!["item".to_string()]);

    root.add_child(item1);
    root.add_child(item2);

    FlexboxEngine::compute_layout(&mut root, &stylesheet, &locale, 800.0, 600.0);

    assert_eq!(root.dimensions.content.width, 500.0);
    assert_eq!(root.children.len(), 2);
    
    // First child x = 0
    assert_eq!(root.children[0].dimensions.content.x, 0.0);
    // Second child x = item1.width (100) + gap (10) = 110.0
    assert_eq!(root.children[1].dimensions.content.x, 110.0);

    Ok(())
}
