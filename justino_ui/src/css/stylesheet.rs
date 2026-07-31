//! CSS Specificity, Cascade and Rule Resolution.

use crate::css::value::CssValue;
use std::collections::HashMap;

/// CSS Pseudo-class types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PseudoClass {
    Hover,
    Active,
    Focus,
    Lang(String),
}

/// Selector specificity tuple `(id, class/pseudo, tag)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectorSpecificity {
    pub ids: usize,
    pub classes: usize,
    pub tags: usize,
}

impl SelectorSpecificity {
    pub fn calculate(
        tag: &Option<String>,
        id: &Option<String>,
        classes: &[String],
        pseudo: &Option<PseudoClass>,
    ) -> Self {
        let ids = if id.is_some() { 1 } else { 0 };
        let class_cnt = classes.len() + if pseudo.is_some() { 1 } else { 0 };
        let tags = if tag.is_some() && tag.as_deref() != Some("*") { 1 } else { 0 };

        Self {
            ids,
            classes: class_cnt,
            tags,
        }
    }
}

/// A CSS Selector matching elements.
#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub pseudo: Option<PseudoClass>,
    pub specificity: SelectorSpecificity,
}

impl Selector {
    pub fn matches(
        &self,
        element_tag: &str,
        element_id: Option<&str>,
        element_classes: &[String],
        active_pseudo: Option<&PseudoClass>,
        current_lang: &str,
    ) -> bool {
        if let Some(t) = &self.tag {
            if t != "*" && !t.eq_ignore_ascii_case(element_tag) {
                return false;
            }
        }

        if let Some(expected_id) = &self.id {
            match element_id {
                Some(actual_id) if actual_id == expected_id => {}
                _ => return false,
            }
        }

        for required_class in &self.classes {
            if !element_classes.contains(required_class) {
                return false;
            }
        }

        if let Some(required_pseudo) = &self.pseudo {
            match required_pseudo {
                PseudoClass::Lang(l) => {
                    if !current_lang.starts_with(l) {
                        return false;
                    }
                }
                other => {
                    if active_pseudo != Some(other) {
                        return false;
                    }
                }
            }
        }

        true
    }
}

/// A CSS Rule comprising selectors and property declarations.
#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: HashMap<String, CssValue>,
}

/// A parsed Stylesheet containing CSS rules.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

impl Stylesheet {
    /// Computes the final cascaded properties map for an element based on selector specificity.
    pub fn compute_style(
        &self,
        element_tag: &str,
        element_id: Option<&str>,
        element_classes: &[String],
        active_pseudo: Option<&PseudoClass>,
        current_lang: &str,
    ) -> HashMap<String, CssValue> {
        let mut matched_rules: Vec<(&Selector, &HashMap<String, CssValue>)> = Vec::new();

        for rule in &self.rules {
            for selector in &rule.selectors {
                if selector.matches(element_tag, element_id, element_classes, active_pseudo, current_lang) {
                    matched_rules.push((selector, &rule.declarations));
                }
            }
        }

        // Sort by specificity ascending so higher specificity overwrites lower
        matched_rules.sort_by(|(s1, _), (s2, _)| s1.specificity.cmp(&s2.specificity));

        let mut final_style = HashMap::new();
        for (_, declarations) in matched_rules {
            for (prop, val) in declarations {
                final_style.insert(prop.clone(), val.clone());
            }
        }

        final_style
    }
}
