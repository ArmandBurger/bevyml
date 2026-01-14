use bevy_log::debug;
use bevy_ui::widget::Text;

use crate::{
    attributes::Attributes,
    inode::{BevyNodeTree, INode, NodeId, NodeType, TextPosition},
    tree_sitter::{Node as TsNode, Tree},
};
use std::{borrow::Cow, convert::TryFrom, fmt};

/// Intermediary Tree
pub struct ITree<'source> {
    pub roots: Vec<NodeId>,
    pub nodes: Vec<INode<'source>>,
    pub child_indices: Vec<NodeId>,
}

impl<'source> fmt::Debug for ITree<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ITree")
            .field("roots", &self.roots)
            .field("nodes_len", &self.nodes.len())
            .field("edges_len", &self.child_indices.len())
            .finish()
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
        let mut itree = ITree::new();
        let roots = collect_root_elements(tree.root_node(), source, &mut itree);
        if roots.is_empty() {
            return Err(ITreeError::MissingRootElement);
        }

        itree.roots = roots;
        Ok(itree)
    }
}

impl<'source> Into<Vec<BevyNodeTree>> for ITree<'source> {
    fn into(self) -> Vec<BevyNodeTree> {
        self.into_bevy_trees()
    }
}

impl<'source> ITree<'source> {
    fn new() -> Self {
        Self {
            roots: Vec::new(),
            nodes: Vec::new(),
            child_indices: Vec::new(),
        }
    }

    pub fn node(&self, id: NodeId) -> &INode<'source> {
        &self.nodes[id.index()]
    }

    pub fn children(&self, id: NodeId) -> &[NodeId] {
        let range = self.nodes[id.index()].children.clone();
        &self.child_indices[range]
    }

    /// Prints a readable representation of the tree as seen in the CLI helper.
    pub fn pretty_print(&self) {
        self.print_nodes(&self.roots, 0);
    }

    /// Logs the same tree layout via Bevy's logging at the `debug` level.
    pub fn pretty_log(&self) {
        self.log_nodes(&self.roots, 0);
    }

    fn print_nodes(&self, nodes: &[NodeId], depth: usize) {
        for node_id in nodes {
            let node = &self.nodes[node_id.index()];
            let indent = "  ".repeat(depth);
            let tag_name = node.node_type.tag_name();
            let element_name = if tag_name.as_ref() == "unknown" {
                "<unknown>"
            } else {
                tag_name.as_ref()
            };
            println!(
                "{}- node_type={:?} element={} simplified_content={:?}",
                indent,
                node.node_type,
                element_name,
                node.simplified_content.as_ref()
            );
            let children = self.children(*node_id);
            self.print_nodes(children, depth + 1);
        }
    }

    fn log_nodes(&self, nodes: &[NodeId], depth: usize) {
        for node_id in nodes {
            let node = &self.nodes[node_id.index()];
            let indent = "  ".repeat(depth);
            let tag_name = node.node_type.tag_name();
            let element_name = if tag_name.as_ref() == "unknown" {
                "<unknown>"
            } else {
                tag_name.as_ref()
            };
            debug!(
                "{}- node_type={:?} element={} simplified_content={:?}",
                indent,
                node.node_type,
                element_name,
                node.simplified_content.as_ref()
            );
            let children = self.children(*node_id);
            self.log_nodes(children, depth + 1);
        }
    }

    fn into_bevy_trees(self) -> Vec<BevyNodeTree> {
        let mut nodes: Vec<Option<INode<'source>>> = self.nodes.into_iter().map(Some).collect();
        let child_indices = self.child_indices;
        self.roots
            .into_iter()
            .map(|root| build_bevy_tree(root, &mut nodes, &child_indices))
            .collect()
    }
}

fn build_ui_node<'tree, 'source>(
    node: TsNode<'tree>,
    source: &'source str,
    itree: &mut ITree<'source>,
    parent: Option<NodeId>,
) -> NodeId {
    let (info_node, is_self_closing) = resolve_element_node(node);
    let node_type = extract_tag_name(info_node, source)
        .as_deref()
        .map(NodeType::from_tag_name)
        .unwrap_or_else(|| NodeType::Custom("unknown".to_string()));
    let attributes = extract_attributes(info_node, source);
    let start = info_node.start_position();
    let end = info_node.end_position();
    let original_text = extract_text_slice(info_node, source);
    let simplified_content = if is_self_closing || info_node.kind() != "element" {
        Cow::Borrowed(original_text)
    } else {
        preview_element_text(info_node, source, original_text)
    };
    let id = NodeId::new(itree.nodes.len());
    itree.nodes.push(INode {
        id,
        node_type,
        attributes,
        start_byte: info_node.start_byte(),
        end_byte: info_node.end_byte(),
        start_position: TextPosition::new(start.column, start.row),
        end_position: TextPosition::new(end.column, end.row),
        simplified_content,
        original_text,
        is_self_closing,
        parent,
        children: 0..0,
        text: None,
    });

    let child_start = itree.child_indices.len();
    if !is_self_closing {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if is_element(child) {
                let child_id = build_ui_node(child, source, itree, Some(id));
                itree.child_indices.push(child_id);
            } else if is_text_node(child) {
                if let Some(child_id) = build_text_node(child, source, itree, Some(id)) {
                    itree.child_indices.push(child_id);
                }
            }
        }
    }
    let child_end = itree.child_indices.len();
    itree.nodes[id.index()].children = child_start..child_end;
    id
}

fn build_bevy_tree<'source>(
    id: NodeId,
    nodes: &mut [Option<INode<'source>>],
    child_indices: &[NodeId],
) -> BevyNodeTree {
    let inode = nodes[id.index()]
        .take()
        .expect("node id should exist once in the arena");
    let children_range = inode.children.clone();
    let children = child_indices[children_range]
        .iter()
        .map(|child_id| build_bevy_tree(*child_id, nodes, child_indices))
        .collect();
    let text = inode
        .text
        .as_ref()
        .map(|content| Text::new(content.as_ref()));
    BevyNodeTree {
        node: inode.to_bundle(),
        text,
        children,
    }
}

fn build_text_node<'tree, 'source>(
    node: TsNode<'tree>,
    source: &'source str,
    itree: &mut ITree<'source>,
    parent: Option<NodeId>,
) -> Option<NodeId> {
    let original_text = extract_text_slice(node, source);
    let trimmed = original_text.trim();
    if trimmed.is_empty() {
        return None;
    }

    let start = node.start_position();
    let end = node.end_position();
    let id = NodeId::new(itree.nodes.len());
    itree.nodes.push(INode {
        id,
        node_type: NodeType::Text,
        attributes: Attributes::default(),
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        start_position: TextPosition::new(start.column, start.row),
        end_position: TextPosition::new(end.column, end.row),
        simplified_content: Cow::Borrowed(trimmed),
        original_text,
        text: Some(Cow::Borrowed(trimmed)),
        is_self_closing: true,
        parent,
        children: 0..0,
    });

    Some(id)
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
    itree: &mut ITree<'source>,
) -> Vec<NodeId> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .filter(|child| is_element(*child))
        .map(|child| build_ui_node(child, source, itree, None))
        .collect()
}

fn is_element<'tree>(node: TsNode<'tree>) -> bool {
    matches!(node.kind(), "element" | "self_closing_element")
}

fn is_text_node<'tree>(node: TsNode<'tree>) -> bool {
    matches!(node.kind(), "text" | "entity" | "plain_ampersand")
}

const TEXT_PREVIEW_LIMIT: usize = 32;

fn preview_element_text<'tree, 'source>(
    node: TsNode<'tree>,
    source: &'source str,
    original_text: &'source str,
) -> Cow<'source, str> {
    let start_tag_text = find_child(node, "start_tag")
        .and_then(|tag| tag.utf8_text(source.as_bytes()).ok())
        .unwrap_or_default();
    let end_tag_text = find_child(node, "end_tag")
        .and_then(|tag| tag.utf8_text(source.as_bytes()).ok())
        .unwrap_or_default();

    if start_tag_text.is_empty() || end_tag_text.is_empty() {
        return Cow::Borrowed(original_text);
    }

    if text_only_preview_ok(node, source) {
        return Cow::Borrowed(original_text);
    }

    if has_inner_content(node) {
        return Cow::Owned(format!("{start_tag_text}...{end_tag_text}"));
    }

    let joined_len = start_tag_text.len() + end_tag_text.len();
    if joined_len == original_text.len() {
        Cow::Borrowed(original_text)
    } else {
        Cow::Owned(format!("{start_tag_text}{end_tag_text}"))
    }
}

fn has_inner_content<'tree>(node: TsNode<'tree>) -> bool {
    node.named_child_count() > 2
}

fn text_only_preview_ok<'tree>(node: TsNode<'tree>, source: &str) -> bool {
    let mut cursor = node.walk();
    let mut text = String::new();
    for child in node.named_children(&mut cursor) {
        let kind = child.kind();
        if kind == "start_tag" || kind == "end_tag" {
            continue;
        }
        if is_element(child) {
            return false;
        }
        if let Ok(child_text) = child.utf8_text(source.as_bytes()) {
            text.push_str(child_text);
        }
    }

    if text.trim().is_empty() {
        return false;
    }

    if text.trim().chars().count() > TEXT_PREVIEW_LIMIT {
        return false;
    }

    true
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

fn extract_attributes<'tree, 'source>(
    node: TsNode<'tree>,
    source: &'source str,
) -> Attributes<Cow<'source, str>> {
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
            attributes.add_raw_attribute(name, value);
        }
    }

    attributes
}

fn parse_attribute<'tree, 'source>(
    node: TsNode<'tree>,
    source: &'source str,
) -> Option<(Cow<'source, str>, Option<Cow<'source, str>>)> {
    let name_node = find_child(node, "attribute_name")?;
    let name = name_node.utf8_text(source.as_bytes()).ok()?;
    let name = Cow::Borrowed(name);
    let value_node = find_child(node, "attribute_value");
    let value = value_node.and_then(|node| extract_attribute_value(node, source));
    Some((name, value))
}

fn extract_attribute_value<'tree, 'source>(
    node: TsNode<'tree>,
    source: &'source str,
) -> Option<Cow<'source, str>> {
    let raw_value = node.utf8_text(source.as_bytes()).ok()?;
    Some(unquote_attribute_value(raw_value))
}

fn unquote_attribute_value<'source>(value: &'source str) -> Cow<'source, str> {
    let bytes = value.as_bytes();
    if bytes.len() >= 2 {
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return Cow::Borrowed(&value[1..value.len() - 1]);
        }
    }
    Cow::Borrowed(value)
}
