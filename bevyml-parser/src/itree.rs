use bevy_math::USizeVec2;

use crate::{
    inode::{INode, NodeType},
    inode_info::INodeInfo,
    tree_sitter::{Node as TsNode, Tree},
};
use std::fmt;

/// Intermediary Tree
pub struct ITree {
    pub root: INode,
}

impl fmt::Debug for ITree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ITree").field("root", &self.root).finish()
    }
}

impl ITree {
    pub fn from_tree(tree: &Tree, source: &str) -> Option<Self> {
        let root = find_first_element(tree.root_node())?;
        Some(Self {
            root: build_ui_node(root, source),
        })
    }
}

fn build_ui_node<'tree>(node: TsNode<'tree>, source: &str) -> INode {
    let node_type = extract_tag_name(node, source)
        .map(|tag| NodeType::from_tag_name(tag.as_str()))
        .unwrap_or_else(|| NodeType::Custom("unknown".to_string()));
    let bevy_node = node_type.to_bevy_node();
    let ts_info = build_ts_info(node, source);
    let mut children = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if is_element(child) {
            children.push(build_ui_node(child, source));
        }
    }

    INode {
        node_type,
        node: bevy_node,
        ts_info,
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

fn find_first_element<'tree>(node: TsNode<'tree>) -> Option<TsNode<'tree>> {
    if is_element(node) {
        return Some(node);
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(found) = find_first_element(child) {
            return Some(found);
        }
    }

    None
}

fn is_element<'tree>(node: TsNode<'tree>) -> bool {
    matches!(node.kind(), "element" | "self_closing_element")
}

fn build_ts_info<'tree>(node: TsNode<'tree>, source: &str) -> INodeInfo {
    let start = node.start_position();
    let end = node.end_position();
    let text = node.utf8_text(source.as_bytes()).unwrap_or("").to_string();

    INodeInfo {
        kind: node.kind().to_string(),
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        start_position: USizeVec2::new(start.row, start.column),
        end_position: USizeVec2::new(end.row, end.column),
        text,
    }
}
