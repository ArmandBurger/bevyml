use bevy_log::debug;
use bevy_math::USizeVec2;

use crate::{
    attributes::Attributes,
    inode::{BevyNodeTree, INode, NodeType},
    tree_sitter::{Node as TsNode, Tree},
};
use std::{convert::TryFrom, fmt};

/// Intermediary Tree
pub struct ITree<'source> {
    pub roots: Vec<INode<'source>>,
}

impl<'source> fmt::Debug for ITree<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ITree").field("roots", &self.roots).finish()
    }
}

#[derive(Debug)]
pub enum ITreeError {
    MissingParseTree,
    MissingRootElement,
}

impl fmt::Display for ITreeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ITreeError::MissingParseTree => write!(f, "parser produced no tree"),
            ITreeError::MissingRootElement => write!(f, "parsed tree contains no element nodes"),
        }
    }
}

impl std::error::Error for ITreeError {}

impl<'source> TryFrom<(&Tree, &'source str)> for ITree<'source> {
    type Error = ITreeError;

    fn try_from((tree, source): (&Tree, &'source str)) -> Result<Self, Self::Error> {
        let roots = collect_root_elements(tree.root_node(), source);
        if roots.is_empty() {
            return Err(ITreeError::MissingRootElement);
        }

        Ok(Self { roots })
    }
}

impl<'source> Into<Vec<BevyNodeTree>> for ITree<'source> {
    fn into(self) -> Vec<BevyNodeTree> {
        self.roots.into_iter().map(BevyNodeTree::from).collect()
    }
}

impl<'source> ITree<'source> {
    /// Prints a readable representation of the tree as seen in the CLI helper.
    pub fn pretty_print(&self) {
        Self::print_nodes(&self.roots, 0);
    }

    /// Logs the same tree layout via Bevy's logging at the `debug` level.
    pub fn pretty_log(&self) {
        Self::log_nodes(&self.roots, 0);
    }

    fn print_nodes(nodes: &[INode<'source>], depth: usize) {
        for node in nodes {
            let indent = "  ".repeat(depth);
            let element_name = node.element_name.as_deref().unwrap_or("<unknown>");
            println!(
                "{}- node_type={:?} element={} simplified_content={:?}",
                indent, node.node_type, element_name, node.simplified_content
            );
            Self::print_nodes(&node.children, depth + 1);
        }
    }

    fn log_nodes(nodes: &[INode<'source>], depth: usize) {
        for node in nodes {
            let indent = "  ".repeat(depth);
            let element_name = node.element_name.as_deref().unwrap_or("<unknown>");
            debug!(
                "{}- node_type={:?} element={} simplified_content={:?}",
                indent, node.node_type, element_name, node.simplified_content
            );
            Self::log_nodes(&node.children, depth + 1);
        }
    }
}

fn build_ui_node<'tree, 'source>(node: TsNode<'tree>, source: &'source str) -> INode<'source> {
    let (info_node, is_self_closing) = resolve_element_node(node);
    let element_name = extract_tag_name(info_node, source);
    let node_type = element_name
        .as_deref()
        .map(NodeType::from_tag_name)
        .unwrap_or_else(|| NodeType::Custom("unknown".to_string()));
    let attributes = extract_attributes(info_node, source);
    let start = info_node.start_position();
    let end = info_node.end_position();
    let simplified_content = if is_self_closing {
        info_node
            .utf8_text(source.as_bytes())
            .unwrap_or("")
            .to_string()
    } else if info_node.kind() == "element" {
        preview_element_text(info_node, source)
    } else {
        info_node
            .utf8_text(source.as_bytes())
            .unwrap_or("")
            .to_string()
    };
    let original_text = extract_text_slice(info_node, source);
    let mut children = Vec::new();
    if !is_self_closing {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if is_element(child) {
                children.push(build_ui_node(child, source));
            }
        }
    }

    INode {
        node_type,
        element_name,
        attributes,
        start_byte: info_node.start_byte(),
        end_byte: info_node.end_byte(),
        start_position: USizeVec2::new(start.row, start.column),
        end_position: USizeVec2::new(end.row, end.column),
        simplified_content,
        original_text,
        is_self_closing,
        children,
    }
}

fn extract_tag_name<'tree>(node: TsNode<'tree>, source: &str) -> Option<String> {
    if node.kind() == "self_closing_element" {
        let tag_node = find_child(node, "tag_name")?;
        let tag_text = tag_node.utf8_text(source.as_bytes()).ok()?;
        return Some(tag_text.to_string());
    }

    if node.kind() == "element" {
        let start_tag = find_child(node, "start_tag")?;
        let tag_node = find_child(start_tag, "tag_name")?;
        let tag_text = tag_node.utf8_text(source.as_bytes()).ok()?;
        return Some(tag_text.to_string());
    }

    None
}

fn find_child<'tree>(node: TsNode<'tree>, kind: &str) -> Option<TsNode<'tree>> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .find(|child| child.kind() == kind)
}

fn collect_root_elements<'tree, 'source>(
    node: TsNode<'tree>,
    source: &'source str,
) -> Vec<INode<'source>> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .filter(|child| is_element(*child))
        .map(|child| build_ui_node(child, source))
        .collect()
}

fn is_element<'tree>(node: TsNode<'tree>) -> bool {
    matches!(node.kind(), "element" | "self_closing_element")
}

fn preview_element_text<'tree>(node: TsNode<'tree>, source: &str) -> String {
    let start_tag_text = find_child(node, "start_tag")
        .and_then(|tag| tag.utf8_text(source.as_bytes()).ok())
        .unwrap_or_default();
    let end_tag_text = find_child(node, "end_tag")
        .and_then(|tag| tag.utf8_text(source.as_bytes()).ok())
        .unwrap_or_default();

    if start_tag_text.is_empty() || end_tag_text.is_empty() {
        return node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
    }

    if has_inner_content(node) {
        format!("{start_tag_text}...{end_tag_text}")
    } else {
        format!("{start_tag_text}{end_tag_text}")
    }
}

fn has_inner_content<'tree>(node: TsNode<'tree>) -> bool {
    node.named_child_count() > 2
}

fn extract_text_slice<'source>(node: TsNode<'_>, source: &'source str) -> &'source str {
    source
        .get(node.start_byte()..node.end_byte())
        .unwrap_or_default()
}

fn resolve_element_node<'tree>(node: TsNode<'tree>) -> (TsNode<'tree>, bool) {
    if node.kind() == "self_closing_element" {
        return (node, true);
    }

    if node.kind() == "element" {
        let has_start_tag = find_child(node, "start_tag").is_some();
        let has_end_tag = find_child(node, "end_tag").is_some();
        if !has_start_tag && !has_end_tag {
            if let Some(self_closing) = find_child(node, "self_closing_element") {
                return (self_closing, true);
            }
        }
    }

    (node, false)
}

fn extract_attributes<'tree>(node: TsNode<'tree>, source: &str) -> Attributes {
    let mut attributes = Attributes::default();
    let attribute_parent = match node.kind() {
        "self_closing_element" => Some(node),
        "element" => find_child(node, "start_tag"),
        _ => None,
    };

    let Some(parent) = attribute_parent else {
        return attributes;
    };

    let mut cursor = parent.walk();
    for child in parent.children(&mut cursor) {
        if child.kind() != "attribute" {
            continue;
        }
        if let Some((name, value)) = parse_attribute(child, source) {
            attributes.add_raw_attribute(&name, value);
        }
    }

    attributes
}

fn parse_attribute<'tree>(node: TsNode<'tree>, source: &str) -> Option<(String, Option<String>)> {
    let name_node = find_child(node, "attribute_name")?;
    let name = name_node.utf8_text(source.as_bytes()).ok()?.to_string();
    let value_node = find_child(node, "attribute_value");
    let value = value_node.and_then(|node| extract_attribute_value(node, source));
    Some((name, value))
}

fn extract_attribute_value<'tree>(node: TsNode<'tree>, source: &str) -> Option<String> {
    let raw_value = node.utf8_text(source.as_bytes()).ok()?;
    Some(unquote_attribute_value(raw_value))
}

fn unquote_attribute_value(value: &str) -> String {
    let bytes = value.as_bytes();
    if bytes.len() >= 2 {
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return value[1..value.len() - 1].to_string();
        }
    }
    value.to_string()
}
