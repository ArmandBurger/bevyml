use std::{borrow::Cow, fmt, ops::Range, str::FromStr};

use bevy_ecs::{bundle::Bundle, component::Component, name::Name};
use bevy_reflect::Reflect;
use bevy_ui::{Display, Node, UiRect, Val};
use strum_macros::{AsRefStr, EnumString};

use crate::attributes::Attributes;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component)]
pub struct NodeId(pub(crate) u32);

impl NodeId {
    pub(crate) fn new(index: usize) -> Self {
        let index = u32::try_from(index).expect("node index overflow");
        Self(index)
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextPosition {
    pub column: usize,
    pub row: usize,
}

impl TextPosition {
    pub const fn new(column: usize, row: usize) -> Self {
        Self { column, row }
    }
}

/// Intermediary Node
pub struct INode<'source> {
    pub id: NodeId,
    pub node_type: NodeType,
    pub attributes: Attributes,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_position: TextPosition,
    pub end_position: TextPosition,
    pub simplified_content: Cow<'source, str>,
    pub original_text: &'source str,
    pub is_self_closing: bool,
    pub parent: Option<NodeId>,
    pub children: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct BevyNodeTree {
    pub node: INodeBundle,
    pub children: Vec<BevyNodeTree>,
}

#[derive(Bundle, Clone)]
pub struct INodeBundle {
    pub id: NodeId,
    pub name: Name,
    pub node: Node,
    pub node_kind: NodeKind,
    pub attributes: Attributes,
}

impl fmt::Debug for INodeBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("INodeBundle")
            .field("name", &self.name)
            // .field("node", &self.node)
            .finish()
    }
}

impl<'source> INode<'source> {
    pub fn to_bundle(&self) -> INodeBundle {
        INodeBundle {
            id: self.id,
            name: Name::new(self.node_type.tag_name().into_owned()),
            node: self.node_type.to_bevy_node(),
            node_kind: NodeKind {
                kind: self.node_type.clone(),
            },
            attributes: self.attributes.clone(),
        }
    }
}

impl<'source> fmt::Debug for INode<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("INode")
            .field("node_type", &self.node_type)
            .field("attributes", &self.attributes)
            .field("simplified_content", &self.simplified_content)
            .field("parent", &self.parent)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq, Reflect, Debug, EnumString, AsRefStr)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum NodeType {
    Html,
    Head,
    Body,
    Title,
    Meta,
    Link,
    Style,
    Script,
    Div,
    Span,
    P,
    A,
    Img,
    Button,
    Input,
    Label,
    Textarea,
    Select,
    Option,
    Ul,
    Ol,
    Li,
    Table,
    Thead,
    Tbody,
    Tfoot,
    Tr,
    Th,
    Td,
    Header,
    Footer,
    Nav,
    Main,
    Section,
    Article,
    Aside,
    Form,
    Canvas,
    Svg,
    Br,
    Hr,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    #[strum(disabled)]
    Custom(String),
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct NodeKind {
    pub kind: NodeType,
}

impl NodeType {
    pub fn from_tag_name(tag_name: &str) -> Self {
        NodeType::from_str(tag_name).unwrap_or_else(|_| NodeType::Custom(tag_name.to_string()))
    }

    pub fn tag_name(&self) -> Cow<'_, str> {
        match self {
            NodeType::Custom(name) => Cow::Borrowed(name.as_str()),
            _ => Cow::Borrowed(self.as_ref()),
        }
    }

    pub fn to_bevy_node(&self) -> Node {
        match self {
            NodeType::Html => block_node(),
            NodeType::Head
            | NodeType::Title
            | NodeType::Meta
            | NodeType::Link
            | NodeType::Style
            | NodeType::Script => Node {
                display: Display::None,
                ..Default::default()
            },
            NodeType::Body => Node {
                display: Display::Block,
                margin: UiRect::all(Val::Px(8.0)),
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                ..Default::default()
            },
            NodeType::Div
            | NodeType::Header
            | NodeType::Footer
            | NodeType::Nav
            | NodeType::Main
            | NodeType::Section
            | NodeType::Article
            | NodeType::Aside
            | NodeType::Form => block_node(),
            NodeType::P => block_with_margin(BASE_FONT_PX),
            NodeType::Ul | NodeType::Ol => Node {
                display: Display::Block,
                margin: margin_block(BASE_FONT_PX),
                padding: UiRect {
                    left: Val::Px(40.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            NodeType::Li => block_node(),
            NodeType::Table
            | NodeType::Thead
            | NodeType::Tbody
            | NodeType::Tfoot
            | NodeType::Tr
            | NodeType::Th
            | NodeType::Td => block_node(),
            NodeType::Hr => Node {
                display: Display::Block,
                margin: margin_block(BASE_FONT_PX * 0.5),
                height: Val::Px(1.0),
                width: Val::Percent(100.0),
                ..Default::default()
            },
            NodeType::H1 => block_with_margin(BASE_FONT_PX * 0.67),
            NodeType::H2 => block_with_margin(BASE_FONT_PX * 0.83),
            NodeType::H3 => block_with_margin(BASE_FONT_PX),
            NodeType::H4 => block_with_margin(BASE_FONT_PX * 1.33),
            NodeType::H5 => block_with_margin(BASE_FONT_PX * 1.67),
            NodeType::H6 => block_with_margin(BASE_FONT_PX * 2.33),
            _ => Node::default(),
        }
    }
}

const BASE_FONT_PX: f32 = 16.0;

fn block_node() -> Node {
    Node {
        display: Display::Block,
        ..Default::default()
    }
}

fn block_with_margin(px: f32) -> Node {
    Node {
        display: Display::Block,
        margin: margin_block(px),
        ..Default::default()
    }
}

fn margin_block(px: f32) -> UiRect {
    UiRect {
        top: Val::Px(px),
        bottom: Val::Px(px),
        ..Default::default()
    }
}
