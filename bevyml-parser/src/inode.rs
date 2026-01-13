use std::fmt;

use bevy_ecs::{bundle::Bundle, component::Component, name::Name};
use bevy_ui::{Display, Node, UiRect, Val};

use crate::inode_info::INodeInfo;

/// Intermediary Node
pub struct INode<'source> {
    pub node_type: NodeType,
    pub element_name: Option<String>,
    pub node: Node,
    pub ts_info: INodeInfo<'source>,
    pub children: Vec<INode<'source>>,
}

#[derive(Debug, Clone)]
pub struct BevyNodeTree {
    pub node: INodeBundle,
    pub children: Vec<BevyNodeTree>,
}

#[derive(Bundle, Clone)]
pub struct INodeBundle {
    pub name: Name,
    pub node: Node,
    pub node_kind: NodeKind,
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
            name: Name::new(self.element_name.clone().unwrap_or("unknown".to_string())),
            node: self.node.clone(),
            node_kind: NodeKind {
                kind: self.node_type.clone(),
            },
        }
    }
}

impl<'source> From<INode<'source>> for BevyNodeTree {
    fn from(inode: INode<'source>) -> Self {
        let children = inode.children.into_iter().map(BevyNodeTree::from).collect();

        BevyNodeTree {
            node: INodeBundle {
                name: Name::new(inode.element_name.unwrap_or("unknown".into())),
                node: inode.node,
                node_kind: NodeKind {
                    kind: inode.node_type,
                },
            },
            children,
        }
    }
}

impl<'source> fmt::Debug for INode<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("INode")
            .field("node_type", &self.node_type)
            .field("element_name", &self.element_name)
            .field("ts_info", &self.ts_info)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq)]
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
    Custom(String),
}

#[derive(Component, Clone, Debug)]
pub struct NodeKind {
    pub kind: NodeType,
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::Custom(tag) => f.debug_tuple("Custom").field(tag).finish(),
            _ => write!(f, "{}", self.as_str()),
        }
    }
}

impl NodeType {
    pub fn from_tag_name(tag_name: &str) -> Self {
        match tag_name.to_ascii_lowercase().as_str() {
            "html" => Self::Html,
            "head" => Self::Head,
            "body" => Self::Body,
            "title" => Self::Title,
            "meta" => Self::Meta,
            "link" => Self::Link,
            "style" => Self::Style,
            "script" => Self::Script,
            "div" => Self::Div,
            "span" => Self::Span,
            "p" => Self::P,
            "a" => Self::A,
            "img" => Self::Img,
            "button" => Self::Button,
            "input" => Self::Input,
            "label" => Self::Label,
            "textarea" => Self::Textarea,
            "select" => Self::Select,
            "option" => Self::Option,
            "ul" => Self::Ul,
            "ol" => Self::Ol,
            "li" => Self::Li,
            "table" => Self::Table,
            "thead" => Self::Thead,
            "tbody" => Self::Tbody,
            "tfoot" => Self::Tfoot,
            "tr" => Self::Tr,
            "th" => Self::Th,
            "td" => Self::Td,
            "header" => Self::Header,
            "footer" => Self::Footer,
            "nav" => Self::Nav,
            "main" => Self::Main,
            "section" => Self::Section,
            "article" => Self::Article,
            "aside" => Self::Aside,
            "form" => Self::Form,
            "canvas" => Self::Canvas,
            "svg" => Self::Svg,
            "br" => Self::Br,
            "hr" => Self::Hr,
            "h1" => Self::H1,
            "h2" => Self::H2,
            "h3" => Self::H3,
            "h4" => Self::H4,
            "h5" => Self::H5,
            "h6" => Self::H6,
            _ => Self::Custom(tag_name.to_string()),
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

    fn as_str(&self) -> &'static str {
        match self {
            NodeType::Html => "Html",
            NodeType::Head => "Head",
            NodeType::Body => "Body",
            NodeType::Title => "Title",
            NodeType::Meta => "Meta",
            NodeType::Link => "Link",
            NodeType::Style => "Style",
            NodeType::Script => "Script",
            NodeType::Div => "Div",
            NodeType::Span => "Span",
            NodeType::P => "P",
            NodeType::A => "A",
            NodeType::Img => "Img",
            NodeType::Button => "Button",
            NodeType::Input => "Input",
            NodeType::Label => "Label",
            NodeType::Textarea => "Textarea",
            NodeType::Select => "Select",
            NodeType::Option => "Option",
            NodeType::Ul => "Ul",
            NodeType::Ol => "Ol",
            NodeType::Li => "Li",
            NodeType::Table => "Table",
            NodeType::Thead => "Thead",
            NodeType::Tbody => "Tbody",
            NodeType::Tfoot => "Tfoot",
            NodeType::Tr => "Tr",
            NodeType::Th => "Th",
            NodeType::Td => "Td",
            NodeType::Header => "Header",
            NodeType::Footer => "Footer",
            NodeType::Nav => "Nav",
            NodeType::Main => "Main",
            NodeType::Section => "Section",
            NodeType::Article => "Article",
            NodeType::Aside => "Aside",
            NodeType::Form => "Form",
            NodeType::Canvas => "Canvas",
            NodeType::Svg => "Svg",
            NodeType::Br => "Br",
            NodeType::Hr => "Hr",
            NodeType::H1 => "H1",
            NodeType::H2 => "H2",
            NodeType::H3 => "H3",
            NodeType::H4 => "H4",
            NodeType::H5 => "H5",
            NodeType::H6 => "H6",
            NodeType::Custom(_) => "Custom",
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
