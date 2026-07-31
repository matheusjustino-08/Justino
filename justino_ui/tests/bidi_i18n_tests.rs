use justino_ui::css::CssParser;
use justino_ui::i18n::{Locale, TextDirection};
use justino_ui::layout::{FlexboxEngine, UiNode};
use justino_ui::UiError;

#[test]
fn test_bidi_direction_detection() {
    let pt = Locale::parse("pt-BR");
    let en = Locale::parse("en-US");
    let ar = Locale::parse("ar-SA");
    let he = Locale::parse("he-IL");

    assert_eq!(pt.direction, TextDirection::Ltr);
    assert_eq!(en.direction, TextDirection::Ltr);
    assert_eq!(ar.direction, TextDirection::Rtl);
    assert_eq!(he.direction, TextDirection::Rtl);
    assert!(ar.is_rtl());
}

#[test]
fn test_bidi_layout_mirroring_ltr_to_rtl() -> Result<(), UiError> {
    let css = r#"
        .header {
            display: flex;
            flex-direction: row;
            width: 400px;
            margin-left: 20px;
        }

        .box {
            width: 100px;
        }
    "#;

    let mut parser = CssParser::new(css);
    let stylesheet = parser.parse()?;

    let locale_ltr = Locale::pt_br();
    let locale_rtl = Locale::ar_sa();

    let mut node_ltr = UiNode::element(1, "div", vec!["header".to_string()]);
    node_ltr.add_child(UiNode::element(2, "div", vec!["box".to_string()]));

    let mut node_rtl = node_ltr.clone();

    FlexboxEngine::compute_layout(&mut node_ltr, &stylesheet, &locale_ltr, 800.0, 600.0);
    FlexboxEngine::compute_layout(&mut node_rtl, &stylesheet, &locale_rtl, 800.0, 600.0);

    // LTR margin-left is on the left
    assert_eq!(node_ltr.dimensions.margin.left, 20.0);
    assert_eq!(node_ltr.dimensions.margin.right, 0.0);

    // RTL margin-left is mirrored to the right side
    assert_eq!(node_rtl.dimensions.margin.right, 20.0);
    assert_eq!(node_rtl.dimensions.margin.left, 0.0);

    Ok(())
}
