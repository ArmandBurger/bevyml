use bevy_math::USizeVec2;

use crate::{
    inode::{BevyNodeTree, INode, NodeType},
    inode_info::INodeInfo,
    tree_sitter::{Node as TsNode, Tree},
};
use std::{convert::TryFrom, fmt};

/// Intermediary Tree
pub struct ITree {
    pub roots: Vec<INode>,
}

impl fmt::Debug for ITree {
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

impl TryFrom<(&Tree, &str)> for ITree {
    type Error = ITreeError;

    fn try_from((tree, source): (&Tree, &str)) -> Result<Self, Self::Error> {
        let roots = collect_root_elements(tree.root_node(), source);
        if roots.is_empty() {
            return Err(ITreeError::MissingRootElement);
        }

        Ok(Self { roots })
    }
}

impl Into<Vec<BevyNodeTree>> for ITree {
    fn into(self) -> Vec<BevyNodeTree> {
        self.roots.into_iter().map(BevyNodeTree::from).collect()
    }
}

fn build_ui_node<'tree>(node: TsNode<'tree>, source: &str) -> INode {
    let element_name = extract_tag_name(node, source);
    let node_type = element_name
        .as_deref()
        .map(NodeType::from_tag_name)
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
        element_name,
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

fn collect_root_elements<'tree>(node: TsNode<'tree>, source: &str) -> Vec<INode> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .filter(|child| is_element(*child))
        .map(|child| build_ui_node(child, source))
        .collect()
}

fn is_element<'tree>(node: TsNode<'tree>) -> bool {
    matches!(node.kind(), "element" | "self_closing_element")
}

fn build_ts_info<'tree>(node: TsNode<'tree>, source: &str) -> INodeInfo {
    let start = node.start_position();
    let end = node.end_position();
    let text = if node.kind() == "element" {
        preview_element_text(node, source)
    } else {
        node.utf8_text(source.as_bytes()).unwrap_or("").to_string()
    };

    INodeInfo {
        kind: node.kind().to_string(),
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        start_position: USizeVec2::new(start.row, start.column),
        end_position: USizeVec2::new(end.row, end.column),
        text,
    }
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
