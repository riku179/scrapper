#[cfg(test)]
mod test;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SelectorTree {
    pub _id: String,
    pub start_url: String,
    pub selectors: Vec<SelectorNode>,
}

impl SelectorTree {
    pub fn new(sitemap_json: String) -> Result<Self, serde_json::Error> {
        let sitemap = SiteMap::new(sitemap_json)?;
        Ok(SelectorTree {
            _id: sitemap._id.clone(),
            start_url: sitemap.start_url[0].clone(),
            selectors: SelectorNode::new(sitemap),
        })
    }

    pub fn from_json(json: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&json)
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SelectorNode {
    pub id: String,
    pub selector_type: SelectorType,
    pub selector: String,
    pub multiple: bool,
    pub children: Vec<SelectorNode>,
}

impl SelectorNode {
    fn new(mut sitemap: SiteMap) -> Vec<Self> {
        build_selector_node(&mut sitemap.selectors, &"_root".into())
    }

    fn from_raw(raw: &RawSelector) -> Self {
        SelectorNode {
            id: raw.id.clone(),
            selector_type: SelectorType::from_str(&raw._type),
            selector: raw.selector.clone(),
            multiple: raw.multiple,
            children: vec![],
        }
    }
}

fn build_selector_node(raw_selectors: &Vec<RawSelector>, parent_id: &String) -> Vec<SelectorNode> {
    let mut children_selectors = vec![];

    for raw_selector in raw_selectors {
        if raw_selector.parent_selectors.contains(parent_id) {
            children_selectors.push(SelectorNode::from_raw(raw_selector));
        }
    }

    for child_selector in &mut children_selectors {
        child_selector
            .children
            .append(&mut build_selector_node(raw_selectors, &child_selector.id));
    }
    children_selectors
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum SelectorType {
    Text,
    Link,
    Image,
    Element,
}

impl SelectorType {
    fn from_str(s: &str) -> Self {
        match s {
            "SelectorText" => SelectorType::Text,
            "SelectorLink" => SelectorType::Link,
            "SelectorImage" => SelectorType::Image,
            "SelectorElement" => SelectorType::Element,
            _ => panic!("unknown selector type"),
        }
    }
}

#[derive(Deserialize, Debug)]
struct SiteMap {
    _id: String,
    #[serde(rename(deserialize = "startUrl"))]
    start_url: Vec<String>,
    selectors: Vec<RawSelector>,
}

impl SiteMap {
    fn new(json: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&json)
    }
}

#[derive(Deserialize, Debug)]
struct RawSelector {
    id: String,
    #[serde(rename(deserialize = "type"))]
    _type: String,
    selector: String,
    multiple: bool,
    #[serde(rename(deserialize = "parentSelectors"))]
    parent_selectors: Vec<String>,
    delay: i32,
}
