use std::fmt;

use bevy_ecs::{bundle::Bundle, name::Name};
use bevy_ui::Node;

use crate::inode_info::INodeInfo;

/// Intermediary Node
pub struct INode {
    pub node_type: NodeType,
    pub element_name: Option<String>,
    pub node: Node,
    pub ts_info: INodeInfo,
    pub children: Vec<INode>,
}

#[derive(Debug)]
pub struct BevyNodeTree {
    pub node: INodeBundle,
    pub children: Vec<BevyNodeTree>,
}

#[derive(Bundle)]
pub struct INodeBundle {
    name: Name,
    node: Node,
}

impl fmt::Debug for INodeBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("INodeBundle")
            .field("name", &self.name)
            // .field("node", &self.node)
            .finish()
    }
}

impl INode {
    pub fn to_bundle(&self) -> INodeBundle {
        INodeBundle {
            name: Name::new(self.element_name.clone().unwrap_or("unknown".to_string())),
            node: Node::default(),
        }
    }
}

impl From<INode> for BevyNodeTree {
    fn from(inode: INode) -> Self {
        let children = inode.children.into_iter().map(BevyNodeTree::from).collect();

        BevyNodeTree {
            node: INodeBundle {
                name: Name::new(inode.element_name.unwrap_or("unknown".into())),
                node: inode.node,
            },
            children,
        }
    }
}

impl fmt::Debug for INode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("INode")
            .field("node_type", &self.node_type)
            .field("element_name", &self.element_name)
            .field("ts_info", &self.ts_info)
            .field("children", &self.children)
            .finish()
    }
}

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
        Node::default()
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
