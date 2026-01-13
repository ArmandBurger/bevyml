use bevy_ecs::component::Component;
use bevy_reflect::Reflect;
use smallvec::SmallVec;
use std::mem::Discriminant;

#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub enum Attribute {
    Id(String),
    Class(String),
    Style(String),
    Title(String),
    Lang(String),
    Dir(String),
    Hidden(bool),
    TabIndex(String),
    Role(String),
    AccessKey(String),
    Draggable(bool),
    ContentEditable(bool),
    SpellCheck(bool),
    InputMode(String),
    EnterKeyHint(String),
    Translate(bool),
    Data { key: String, value: Option<String> },
    Enabled(bool),
    Disabled(bool),
    Checked(bool),
    Selected(bool),
    ReadOnly(bool),
    Required(bool),
    Multiple(bool),
    Autofocus(bool),
    Href(String),
    Src(String),
    Alt(String),
    Name(String),
    Value(String),
    Type(String),
    Placeholder(String),
    Min(String),
    Max(String),
    Step(String),
    Width(String),
    Height(String),
    Rows(String),
    Cols(String),
    Size(String),
    MaxLength(String),
    MinLength(String),
    Pattern(String),
    Accept(String),
    AcceptCharset(String),
    AutoComplete(String),
    AutoCapitalize(String),
    For(String),
    Action(String),
    Method(String),
    Enctype(String),
    Target(String),
    Rel(String),
    Download(Option<String>),
    SrcSet(String),
    Sizes(String),
    Media(String),
    Loading(String),
    Decoding(String),
    ReferrerPolicy(String),
    CrossOrigin(String),
    Async(bool),
    Defer(bool),
    Charset(String),
    Content(String),
    HttpEquiv(String),
    Controls(bool),
    Autoplay(bool),
    Loop(bool),
    Muted(bool),
    PlaysInline(bool),
    Poster(String),
    Preload(String),
    Aria { name: String, value: Option<String> },
    Custom { name: String, value: Option<String> },
}

#[derive(Component, Reflect, Clone, Debug, Default, PartialEq, Eq)]
pub struct Attributes {
    pub items: SmallVec<[Attribute; 4]>,
    #[reflect(ignore)]
    index: SmallVec<[(Discriminant<Attribute>, usize); 8]>,
}

impl Attributes {
    pub fn add_raw_attribute(&mut self, name: &str, value: Option<String>) {
        let normalized = name.to_ascii_lowercase();
        let bool_value = parse_bool_attribute(value.as_deref());
        match normalized.as_str() {
            "id" => self.push_attribute(Attribute::Id(value.unwrap_or_default())),
            "class" => self.push_attribute(Attribute::Class(value.unwrap_or_default())),
            "style" => self.push_attribute(Attribute::Style(value.unwrap_or_default())),
            "title" => self.push_attribute(Attribute::Title(value.unwrap_or_default())),
            "lang" => self.push_attribute(Attribute::Lang(value.unwrap_or_default())),
            "dir" => self.push_attribute(Attribute::Dir(value.unwrap_or_default())),
            "hidden" => self.push_attribute(Attribute::Hidden(bool_value)),
            "tabindex" => self.push_attribute(Attribute::TabIndex(value.unwrap_or_default())),
            "role" => self.push_attribute(Attribute::Role(value.unwrap_or_default())),
            "accesskey" => self.push_attribute(Attribute::AccessKey(value.unwrap_or_default())),
            "draggable" => self.push_attribute(Attribute::Draggable(bool_value)),
            "contenteditable" => self.push_attribute(Attribute::ContentEditable(bool_value)),
            "spellcheck" => self.push_attribute(Attribute::SpellCheck(bool_value)),
            "inputmode" => self.push_attribute(Attribute::InputMode(value.unwrap_or_default())),
            "enterkeyhint" => {
                self.push_attribute(Attribute::EnterKeyHint(value.unwrap_or_default()))
            }
            "translate" => self.push_attribute(Attribute::Translate(bool_value)),
            "enabled" => self.push_attribute(Attribute::Enabled(bool_value)),
            "disabled" => self.push_attribute(Attribute::Disabled(bool_value)),
            "checked" => self.push_attribute(Attribute::Checked(bool_value)),
            "selected" => self.push_attribute(Attribute::Selected(bool_value)),
            "readonly" => self.push_attribute(Attribute::ReadOnly(bool_value)),
            "required" => self.push_attribute(Attribute::Required(bool_value)),
            "multiple" => self.push_attribute(Attribute::Multiple(bool_value)),
            "autofocus" => self.push_attribute(Attribute::Autofocus(bool_value)),
            "href" => self.push_attribute(Attribute::Href(value.unwrap_or_default())),
            "src" => self.push_attribute(Attribute::Src(value.unwrap_or_default())),
            "alt" => self.push_attribute(Attribute::Alt(value.unwrap_or_default())),
            "name" => self.push_attribute(Attribute::Name(value.unwrap_or_default())),
            "value" => self.push_attribute(Attribute::Value(value.unwrap_or_default())),
            "type" => self.push_attribute(Attribute::Type(value.unwrap_or_default())),
            "placeholder" => self.push_attribute(Attribute::Placeholder(value.unwrap_or_default())),
            "min" => self.push_attribute(Attribute::Min(value.unwrap_or_default())),
            "max" => self.push_attribute(Attribute::Max(value.unwrap_or_default())),
            "step" => self.push_attribute(Attribute::Step(value.unwrap_or_default())),
            "width" => self.push_attribute(Attribute::Width(value.unwrap_or_default())),
            "height" => self.push_attribute(Attribute::Height(value.unwrap_or_default())),
            "rows" => self.push_attribute(Attribute::Rows(value.unwrap_or_default())),
            "cols" => self.push_attribute(Attribute::Cols(value.unwrap_or_default())),
            "size" => self.push_attribute(Attribute::Size(value.unwrap_or_default())),
            "maxlength" => self.push_attribute(Attribute::MaxLength(value.unwrap_or_default())),
            "minlength" => self.push_attribute(Attribute::MinLength(value.unwrap_or_default())),
            "pattern" => self.push_attribute(Attribute::Pattern(value.unwrap_or_default())),
            "accept" => self.push_attribute(Attribute::Accept(value.unwrap_or_default())),
            "accept-charset" => {
                self.push_attribute(Attribute::AcceptCharset(value.unwrap_or_default()))
            }
            "autocomplete" => {
                self.push_attribute(Attribute::AutoComplete(value.unwrap_or_default()))
            }
            "autocapitalize" => {
                self.push_attribute(Attribute::AutoCapitalize(value.unwrap_or_default()))
            }
            "for" => self.push_attribute(Attribute::For(value.unwrap_or_default())),
            "action" => self.push_attribute(Attribute::Action(value.unwrap_or_default())),
            "method" => self.push_attribute(Attribute::Method(value.unwrap_or_default())),
            "enctype" => self.push_attribute(Attribute::Enctype(value.unwrap_or_default())),
            "target" => self.push_attribute(Attribute::Target(value.unwrap_or_default())),
            "rel" => self.push_attribute(Attribute::Rel(value.unwrap_or_default())),
            "download" => self.push_attribute(Attribute::Download(value)),
            "srcset" => self.push_attribute(Attribute::SrcSet(value.unwrap_or_default())),
            "sizes" => self.push_attribute(Attribute::Sizes(value.unwrap_or_default())),
            "media" => self.push_attribute(Attribute::Media(value.unwrap_or_default())),
            "loading" => self.push_attribute(Attribute::Loading(value.unwrap_or_default())),
            "decoding" => self.push_attribute(Attribute::Decoding(value.unwrap_or_default())),
            "referrerpolicy" => {
                self.push_attribute(Attribute::ReferrerPolicy(value.unwrap_or_default()))
            }
            "crossorigin" => self.push_attribute(Attribute::CrossOrigin(value.unwrap_or_default())),
            "async" => self.push_attribute(Attribute::Async(bool_value)),
            "defer" => self.push_attribute(Attribute::Defer(bool_value)),
            "charset" => self.push_attribute(Attribute::Charset(value.unwrap_or_default())),
            "content" => self.push_attribute(Attribute::Content(value.unwrap_or_default())),
            "http-equiv" => self.push_attribute(Attribute::HttpEquiv(value.unwrap_or_default())),
            "controls" => self.push_attribute(Attribute::Controls(bool_value)),
            "autoplay" => self.push_attribute(Attribute::Autoplay(bool_value)),
            "loop" => self.push_attribute(Attribute::Loop(bool_value)),
            "muted" => self.push_attribute(Attribute::Muted(bool_value)),
            "playsinline" => self.push_attribute(Attribute::PlaysInline(bool_value)),
            "poster" => self.push_attribute(Attribute::Poster(value.unwrap_or_default())),
            "preload" => self.push_attribute(Attribute::Preload(value.unwrap_or_default())),
            _ if normalized.starts_with("data-") => {
                let key = name.get(5..).unwrap_or("").to_string();
                self.push_attribute(Attribute::Data { key, value });
            }
            _ if normalized.starts_with("aria-") => {
                let key = name.get(5..).unwrap_or("").to_string();
                self.push_attribute(Attribute::Aria { name: key, value });
            }
            _ => self.push_attribute(Attribute::Custom {
                name: name.to_string(),
                value,
            }),
        }
    }

    fn push_attribute(&mut self, attribute: Attribute) {
        if attribute.is_multi() {
            self.items.push(attribute);
            return;
        }
        let discriminant = std::mem::discriminant(&attribute);
        if let Some((_, index)) = self
            .index
            .iter_mut()
            .find(|(entry, _)| *entry == discriminant)
        {
            self.items[*index] = attribute;
            return;
        }

        let index = self.items.len();
        self.items.push(attribute);
        self.index.push((discriminant, index));
    }
}

fn parse_bool_attribute(value: Option<&str>) -> bool {
    match value {
        None => true,
        Some(raw) => {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                return true;
            }
            let lowered = trimmed.to_ascii_lowercase();
            !matches!(lowered.as_str(), "false" | "0" | "no" | "off")
        }
    }
}

impl Attribute {
    fn is_multi(&self) -> bool {
        matches!(
            self,
            Attribute::Data { .. } | Attribute::Aria { .. } | Attribute::Custom { .. }
        )
    }
}
