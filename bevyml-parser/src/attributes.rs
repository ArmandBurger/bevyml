use bevy_color::{palettes::basic, Color, Srgba};
use bevy_ecs::component::Component;
use bevy_log::warn;
use bevy_reflect::Reflect;
use bevy_ui::{AlignItems, BorderRadius, Display, JustifyContent, UiRect, Val};
use smallvec::SmallVec;
use std::{borrow::Cow, mem::Discriminant};

#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub struct ClassList<Str = String> {
    pub raw: Str,
    pub classes: SmallVec<[Str; 4]>,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct StyleAttribute<Str = String> {
    pub raw: Str,
    pub declarations: SmallVec<[StyleDeclaration; 8]>,
    pub unsupported: SmallVec<[UnsupportedStyle<Str>; 4]>,
}

#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub struct UnsupportedStyle<Str = String> {
    pub property: Str,
    pub value: Str,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct BorderStyle {
    pub thickness: UiRect,
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum StyleDeclaration {
    Display(Display),
    Width(Val),
    Height(Val),
    MinWidth(Val),
    MaxWidth(Val),
    MinHeight(Val),
    MaxHeight(Val),
    Left(Val),
    Right(Val),
    Top(Val),
    Bottom(Val),
    Margin(UiRect),
    MarginLeft(Val),
    MarginRight(Val),
    MarginTop(Val),
    MarginBottom(Val),
    Padding(UiRect),
    PaddingLeft(Val),
    PaddingRight(Val),
    PaddingTop(Val),
    PaddingBottom(Val),
    Border(BorderStyle),
    BorderLeft(Val),
    BorderRight(Val),
    BorderTop(Val),
    BorderBottom(Val),
    BorderRadius(BorderRadius),
    BackgroundColor(Color),
    AlignItems(AlignItems),
    JustifyContent(JustifyContent),
    RowGap(Val),
    ColumnGap(Val),
    Gap { row: Val, column: Val },
    FlexBasis(Val),
}

#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum Attribute<Str = String> {
    Id(Str),
    Class(ClassList<Str>),
    Style(StyleAttribute<Str>),
    Title(Str),
    Lang(Str),
    Dir(Str),
    Hidden(bool),
    TabIndex(Str),
    Role(Str),
    AccessKey(Str),
    Draggable(bool),
    ContentEditable(bool),
    SpellCheck(bool),
    InputMode(Str),
    EnterKeyHint(Str),
    Translate(bool),
    Data { key: Str, value: Option<Str> },
    Enabled(bool),
    Disabled(bool),
    Checked(bool),
    Selected(bool),
    ReadOnly(bool),
    Required(bool),
    Multiple(bool),
    Autofocus(bool),
    Href(Str),
    Src(Str),
    Alt(Str),
    Name(Str),
    Value(Str),
    Type(Str),
    Placeholder(Str),
    Min(Str),
    Max(Str),
    Step(Str),
    Width(Str),
    Height(Str),
    Rows(Str),
    Cols(Str),
    Size(Str),
    MaxLength(Str),
    MinLength(Str),
    Pattern(Str),
    Accept(Str),
    AcceptCharset(Str),
    AutoComplete(Str),
    AutoCapitalize(Str),
    For(Str),
    Action(Str),
    Method(Str),
    Enctype(Str),
    Target(Str),
    Rel(Str),
    Download(Option<Str>),
    SrcSet(Str),
    Sizes(Str),
    Media(Str),
    Loading(Str),
    Decoding(Str),
    ReferrerPolicy(Str),
    CrossOrigin(Str),
    Async(bool),
    Defer(bool),
    Charset(Str),
    Content(Str),
    HttpEquiv(Str),
    Controls(bool),
    Autoplay(bool),
    Loop(bool),
    Muted(bool),
    PlaysInline(bool),
    Poster(Str),
    Preload(Str),
    Aria { name: Str, value: Option<Str> },
    Custom { name: Str, value: Option<Str> },
}

#[derive(Component, Reflect, Clone, Debug, Default, PartialEq)]
pub struct Attributes<Str = String> {
    pub items: SmallVec<[Attribute<Str>; 4]>,
    #[reflect(ignore)]
    index: SmallVec<[(Discriminant<Attribute<Str>>, usize); 8]>,
}

impl<'a> Attributes<Cow<'a, str>> {
    pub fn add_raw_attribute(&mut self, name: Cow<'a, str>, value: Option<Cow<'a, str>>) {
        let attribute = build_attribute(name, value);
        self.push_attribute(attribute);
    }

    pub fn into_owned(self) -> Attributes<String> {
        let mut owned = Attributes::default();
        for attribute in self.items {
            owned.push_attribute(attribute.into_owned());
        }
        owned
    }
}

impl Attributes<String> {
    pub fn add_raw_attribute(&mut self, name: &str, value: Option<String>) {
        let attribute = build_attribute(Cow::Owned(name.to_string()), value.map(Cow::Owned));
        self.push_attribute(attribute.into_owned());
    }
}

impl<Str> Attributes<Str> {
    fn push_attribute(&mut self, attribute: Attribute<Str>) {
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

fn build_attribute<'a>(name: Cow<'a, str>, value: Option<Cow<'a, str>>) -> Attribute<Cow<'a, str>> {
    let normalized = name.as_ref().to_ascii_lowercase();
    let bool_value = parse_bool_attribute(value.as_deref());
    match normalized.as_str() {
        "id" => Attribute::Id(value.unwrap_or_else(empty_cow)),
        "class" => Attribute::Class(ClassList::parse(value.unwrap_or_else(empty_cow))),
        "style" => Attribute::Style(StyleAttribute::parse(value.unwrap_or_else(empty_cow))),
        "title" => Attribute::Title(value.unwrap_or_else(empty_cow)),
        "lang" => Attribute::Lang(value.unwrap_or_else(empty_cow)),
        "dir" => Attribute::Dir(value.unwrap_or_else(empty_cow)),
        "hidden" => Attribute::Hidden(bool_value),
        "tabindex" => Attribute::TabIndex(value.unwrap_or_else(empty_cow)),
        "role" => Attribute::Role(value.unwrap_or_else(empty_cow)),
        "accesskey" => Attribute::AccessKey(value.unwrap_or_else(empty_cow)),
        "draggable" => Attribute::Draggable(bool_value),
        "contenteditable" => Attribute::ContentEditable(bool_value),
        "spellcheck" => Attribute::SpellCheck(bool_value),
        "inputmode" => Attribute::InputMode(value.unwrap_or_else(empty_cow)),
        "enterkeyhint" => Attribute::EnterKeyHint(value.unwrap_or_else(empty_cow)),
        "translate" => Attribute::Translate(bool_value),
        "enabled" => Attribute::Enabled(bool_value),
        "disabled" => Attribute::Disabled(bool_value),
        "checked" => Attribute::Checked(bool_value),
        "selected" => Attribute::Selected(bool_value),
        "readonly" => Attribute::ReadOnly(bool_value),
        "required" => Attribute::Required(bool_value),
        "multiple" => Attribute::Multiple(bool_value),
        "autofocus" => Attribute::Autofocus(bool_value),
        "href" => Attribute::Href(value.unwrap_or_else(empty_cow)),
        "src" => Attribute::Src(value.unwrap_or_else(empty_cow)),
        "alt" => Attribute::Alt(value.unwrap_or_else(empty_cow)),
        "name" => Attribute::Name(value.unwrap_or_else(empty_cow)),
        "value" => Attribute::Value(value.unwrap_or_else(empty_cow)),
        "type" => Attribute::Type(value.unwrap_or_else(empty_cow)),
        "placeholder" => Attribute::Placeholder(value.unwrap_or_else(empty_cow)),
        "min" => Attribute::Min(value.unwrap_or_else(empty_cow)),
        "max" => Attribute::Max(value.unwrap_or_else(empty_cow)),
        "step" => Attribute::Step(value.unwrap_or_else(empty_cow)),
        "width" => Attribute::Width(value.unwrap_or_else(empty_cow)),
        "height" => Attribute::Height(value.unwrap_or_else(empty_cow)),
        "rows" => Attribute::Rows(value.unwrap_or_else(empty_cow)),
        "cols" => Attribute::Cols(value.unwrap_or_else(empty_cow)),
        "size" => Attribute::Size(value.unwrap_or_else(empty_cow)),
        "maxlength" => Attribute::MaxLength(value.unwrap_or_else(empty_cow)),
        "minlength" => Attribute::MinLength(value.unwrap_or_else(empty_cow)),
        "pattern" => Attribute::Pattern(value.unwrap_or_else(empty_cow)),
        "accept" => Attribute::Accept(value.unwrap_or_else(empty_cow)),
        "accept-charset" => Attribute::AcceptCharset(value.unwrap_or_else(empty_cow)),
        "autocomplete" => Attribute::AutoComplete(value.unwrap_or_else(empty_cow)),
        "autocapitalize" => Attribute::AutoCapitalize(value.unwrap_or_else(empty_cow)),
        "for" => Attribute::For(value.unwrap_or_else(empty_cow)),
        "action" => Attribute::Action(value.unwrap_or_else(empty_cow)),
        "method" => Attribute::Method(value.unwrap_or_else(empty_cow)),
        "enctype" => Attribute::Enctype(value.unwrap_or_else(empty_cow)),
        "target" => Attribute::Target(value.unwrap_or_else(empty_cow)),
        "rel" => Attribute::Rel(value.unwrap_or_else(empty_cow)),
        "download" => Attribute::Download(value),
        "srcset" => Attribute::SrcSet(value.unwrap_or_else(empty_cow)),
        "sizes" => Attribute::Sizes(value.unwrap_or_else(empty_cow)),
        "media" => Attribute::Media(value.unwrap_or_else(empty_cow)),
        "loading" => Attribute::Loading(value.unwrap_or_else(empty_cow)),
        "decoding" => Attribute::Decoding(value.unwrap_or_else(empty_cow)),
        "referrerpolicy" => Attribute::ReferrerPolicy(value.unwrap_or_else(empty_cow)),
        "crossorigin" => Attribute::CrossOrigin(value.unwrap_or_else(empty_cow)),
        "async" => Attribute::Async(bool_value),
        "defer" => Attribute::Defer(bool_value),
        "charset" => Attribute::Charset(value.unwrap_or_else(empty_cow)),
        "content" => Attribute::Content(value.unwrap_or_else(empty_cow)),
        "http-equiv" => Attribute::HttpEquiv(value.unwrap_or_else(empty_cow)),
        "controls" => Attribute::Controls(bool_value),
        "autoplay" => Attribute::Autoplay(bool_value),
        "loop" => Attribute::Loop(bool_value),
        "muted" => Attribute::Muted(bool_value),
        "playsinline" => Attribute::PlaysInline(bool_value),
        "poster" => Attribute::Poster(value.unwrap_or_else(empty_cow)),
        "preload" => Attribute::Preload(value.unwrap_or_else(empty_cow)),
        _ if normalized.starts_with("data-") => {
            let key = match &name {
                Cow::Borrowed(raw) => Cow::Borrowed(raw.get(5..).unwrap_or("")),
                Cow::Owned(raw) => Cow::Owned(raw.get(5..).unwrap_or("").to_string()),
            };
            Attribute::Data { key, value }
        }
        _ if normalized.starts_with("aria-") => {
            let key = match &name {
                Cow::Borrowed(raw) => Cow::Borrowed(raw.get(5..).unwrap_or("")),
                Cow::Owned(raw) => Cow::Owned(raw.get(5..).unwrap_or("").to_string()),
            };
            Attribute::Aria { name: key, value }
        }
        _ => Attribute::Custom { name, value },
    }
}

fn empty_cow<'a>() -> Cow<'a, str> {
    Cow::Borrowed("")
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

impl<Str> Attribute<Str> {
    fn is_multi(&self) -> bool {
        matches!(
            self,
            Attribute::Data { .. } | Attribute::Aria { .. } | Attribute::Custom { .. }
        )
    }
}

impl<'a> Attribute<Cow<'a, str>> {
    pub fn into_owned(self) -> Attribute<String> {
        match self {
            Attribute::Id(value) => Attribute::Id(value.into_owned()),
            Attribute::Class(value) => Attribute::Class(value.into_owned()),
            Attribute::Style(value) => Attribute::Style(value.into_owned()),
            Attribute::Title(value) => Attribute::Title(value.into_owned()),
            Attribute::Lang(value) => Attribute::Lang(value.into_owned()),
            Attribute::Dir(value) => Attribute::Dir(value.into_owned()),
            Attribute::Hidden(value) => Attribute::Hidden(value),
            Attribute::TabIndex(value) => Attribute::TabIndex(value.into_owned()),
            Attribute::Role(value) => Attribute::Role(value.into_owned()),
            Attribute::AccessKey(value) => Attribute::AccessKey(value.into_owned()),
            Attribute::Draggable(value) => Attribute::Draggable(value),
            Attribute::ContentEditable(value) => Attribute::ContentEditable(value),
            Attribute::SpellCheck(value) => Attribute::SpellCheck(value),
            Attribute::InputMode(value) => Attribute::InputMode(value.into_owned()),
            Attribute::EnterKeyHint(value) => Attribute::EnterKeyHint(value.into_owned()),
            Attribute::Translate(value) => Attribute::Translate(value),
            Attribute::Data { key, value } => Attribute::Data {
                key: key.into_owned(),
                value: value.map(Cow::into_owned),
            },
            Attribute::Enabled(value) => Attribute::Enabled(value),
            Attribute::Disabled(value) => Attribute::Disabled(value),
            Attribute::Checked(value) => Attribute::Checked(value),
            Attribute::Selected(value) => Attribute::Selected(value),
            Attribute::ReadOnly(value) => Attribute::ReadOnly(value),
            Attribute::Required(value) => Attribute::Required(value),
            Attribute::Multiple(value) => Attribute::Multiple(value),
            Attribute::Autofocus(value) => Attribute::Autofocus(value),
            Attribute::Href(value) => Attribute::Href(value.into_owned()),
            Attribute::Src(value) => Attribute::Src(value.into_owned()),
            Attribute::Alt(value) => Attribute::Alt(value.into_owned()),
            Attribute::Name(value) => Attribute::Name(value.into_owned()),
            Attribute::Value(value) => Attribute::Value(value.into_owned()),
            Attribute::Type(value) => Attribute::Type(value.into_owned()),
            Attribute::Placeholder(value) => Attribute::Placeholder(value.into_owned()),
            Attribute::Min(value) => Attribute::Min(value.into_owned()),
            Attribute::Max(value) => Attribute::Max(value.into_owned()),
            Attribute::Step(value) => Attribute::Step(value.into_owned()),
            Attribute::Width(value) => Attribute::Width(value.into_owned()),
            Attribute::Height(value) => Attribute::Height(value.into_owned()),
            Attribute::Rows(value) => Attribute::Rows(value.into_owned()),
            Attribute::Cols(value) => Attribute::Cols(value.into_owned()),
            Attribute::Size(value) => Attribute::Size(value.into_owned()),
            Attribute::MaxLength(value) => Attribute::MaxLength(value.into_owned()),
            Attribute::MinLength(value) => Attribute::MinLength(value.into_owned()),
            Attribute::Pattern(value) => Attribute::Pattern(value.into_owned()),
            Attribute::Accept(value) => Attribute::Accept(value.into_owned()),
            Attribute::AcceptCharset(value) => Attribute::AcceptCharset(value.into_owned()),
            Attribute::AutoComplete(value) => Attribute::AutoComplete(value.into_owned()),
            Attribute::AutoCapitalize(value) => Attribute::AutoCapitalize(value.into_owned()),
            Attribute::For(value) => Attribute::For(value.into_owned()),
            Attribute::Action(value) => Attribute::Action(value.into_owned()),
            Attribute::Method(value) => Attribute::Method(value.into_owned()),
            Attribute::Enctype(value) => Attribute::Enctype(value.into_owned()),
            Attribute::Target(value) => Attribute::Target(value.into_owned()),
            Attribute::Rel(value) => Attribute::Rel(value.into_owned()),
            Attribute::Download(value) => Attribute::Download(value.map(Cow::into_owned)),
            Attribute::SrcSet(value) => Attribute::SrcSet(value.into_owned()),
            Attribute::Sizes(value) => Attribute::Sizes(value.into_owned()),
            Attribute::Media(value) => Attribute::Media(value.into_owned()),
            Attribute::Loading(value) => Attribute::Loading(value.into_owned()),
            Attribute::Decoding(value) => Attribute::Decoding(value.into_owned()),
            Attribute::ReferrerPolicy(value) => Attribute::ReferrerPolicy(value.into_owned()),
            Attribute::CrossOrigin(value) => Attribute::CrossOrigin(value.into_owned()),
            Attribute::Async(value) => Attribute::Async(value),
            Attribute::Defer(value) => Attribute::Defer(value),
            Attribute::Charset(value) => Attribute::Charset(value.into_owned()),
            Attribute::Content(value) => Attribute::Content(value.into_owned()),
            Attribute::HttpEquiv(value) => Attribute::HttpEquiv(value.into_owned()),
            Attribute::Controls(value) => Attribute::Controls(value),
            Attribute::Autoplay(value) => Attribute::Autoplay(value),
            Attribute::Loop(value) => Attribute::Loop(value),
            Attribute::Muted(value) => Attribute::Muted(value),
            Attribute::PlaysInline(value) => Attribute::PlaysInline(value),
            Attribute::Poster(value) => Attribute::Poster(value.into_owned()),
            Attribute::Preload(value) => Attribute::Preload(value.into_owned()),
            Attribute::Aria { name, value } => Attribute::Aria {
                name: name.into_owned(),
                value: value.map(Cow::into_owned),
            },
            Attribute::Custom { name, value } => Attribute::Custom {
                name: name.into_owned(),
                value: value.map(Cow::into_owned),
            },
        }
    }
}

impl<'a> ClassList<Cow<'a, str>> {
    pub fn parse(raw: Cow<'a, str>) -> Self {
        match raw {
            Cow::Borrowed(raw_ref) => {
                let mut classes = SmallVec::new();
                for class in raw_ref.split_whitespace() {
                    if class.is_empty() {
                        continue;
                    }
                    classes.push(Cow::Borrowed(class));
                }
                Self {
                    raw: Cow::Borrowed(raw_ref),
                    classes,
                }
            }
            Cow::Owned(raw_string) => {
                let mut classes = SmallVec::new();
                for class in raw_string.split_whitespace() {
                    if class.is_empty() {
                        continue;
                    }
                    classes.push(Cow::Owned(class.to_string()));
                }
                Self {
                    raw: Cow::Owned(raw_string),
                    classes,
                }
            }
        }
    }

    pub fn into_owned(self) -> ClassList<String> {
        ClassList {
            raw: self.raw.into_owned(),
            classes: self.classes.into_iter().map(Cow::into_owned).collect(),
        }
    }
}

impl<'a> StyleAttribute<Cow<'a, str>> {
    pub fn parse(raw: Cow<'a, str>) -> Self {
        match raw {
            Cow::Borrowed(raw_ref) => parse_style_borrowed(raw_ref),
            Cow::Owned(raw_string) => parse_style_owned(raw_string),
        }
    }

    pub fn into_owned(self) -> StyleAttribute<String> {
        StyleAttribute {
            raw: self.raw.into_owned(),
            declarations: self.declarations,
            unsupported: self
                .unsupported
                .into_iter()
                .map(UnsupportedStyle::into_owned)
                .collect(),
        }
    }
}

impl<'a> UnsupportedStyle<Cow<'a, str>> {
    fn into_owned(self) -> UnsupportedStyle<String> {
        UnsupportedStyle {
            property: self.property.into_owned(),
            value: self.value.into_owned(),
        }
    }
}

fn parse_style_borrowed<'a>(raw: &'a str) -> StyleAttribute<Cow<'a, str>> {
    let mut declarations = SmallVec::new();
    let mut unsupported = SmallVec::new();
    let mut push_unsupported = |property: &str, value: &str| {
        unsupported.push(UnsupportedStyle {
            property: Cow::Owned(property.to_string()),
            value: Cow::Owned(value.to_string()),
        });
    };
    for declaration in raw.split(';') {
        let trimmed = declaration.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some((name_raw, value_raw)) = trimmed.split_once(':') else {
            warn!("style declaration missing ':' -> {:?}", trimmed);
            push_unsupported(trimmed, "");
            continue;
        };
        let name_raw = name_raw.trim();
        let mut value_raw = value_raw.trim();
        if name_raw.is_empty() {
            warn!("style declaration missing property name -> {:?}", trimmed);
            continue;
        }
        value_raw = strip_important(value_raw);
        if value_raw.is_empty() {
            warn!("style declaration missing value for '{}'", name_raw);
            push_unsupported(name_raw, value_raw);
            continue;
        }
        let name_lower = name_raw.to_ascii_lowercase();
        parse_style_property(
            name_raw,
            &name_lower,
            value_raw,
            &mut declarations,
            &mut push_unsupported,
        );
    }
    StyleAttribute {
        raw: Cow::Borrowed(raw),
        declarations,
        unsupported,
    }
}

fn parse_style_owned<'a>(raw: String) -> StyleAttribute<Cow<'a, str>> {
    let mut declarations = SmallVec::new();
    let mut unsupported = SmallVec::new();
    let mut push_unsupported = |property: &str, value: &str| {
        unsupported.push(UnsupportedStyle {
            property: Cow::Owned(property.to_string()),
            value: Cow::Owned(value.to_string()),
        });
    };
    for declaration in raw.split(';') {
        let trimmed = declaration.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some((name_raw, value_raw)) = trimmed.split_once(':') else {
            warn!("style declaration missing ':' -> {:?}", trimmed);
            push_unsupported(trimmed, "");
            continue;
        };
        let name_raw = name_raw.trim();
        let mut value_raw = value_raw.trim();
        if name_raw.is_empty() {
            warn!("style declaration missing property name -> {:?}", trimmed);
            continue;
        }
        value_raw = strip_important(value_raw);
        if value_raw.is_empty() {
            warn!("style declaration missing value for '{}'", name_raw);
            push_unsupported(name_raw, value_raw);
            continue;
        }
        let name_lower = name_raw.to_ascii_lowercase();
        parse_style_property(
            name_raw,
            &name_lower,
            value_raw,
            &mut declarations,
            &mut push_unsupported,
        );
    }
    StyleAttribute {
        raw: Cow::Owned(raw),
        declarations,
        unsupported,
    }
}

fn parse_style_property<F>(
    name_raw: &str,
    name_lower: &str,
    value: &str,
    declarations: &mut SmallVec<[StyleDeclaration; 8]>,
    push_unsupported: &mut F,
) where
    F: FnMut(&str, &str),
{
    match name_lower {
        "width" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::Width,
        ),
        "display" => apply_display_property(name_raw, value, declarations, push_unsupported),
        "height" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::Height,
        ),
        "min-width" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::MinWidth,
        ),
        "max-width" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::MaxWidth,
        ),
        "min-height" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::MinHeight,
        ),
        "max-height" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::MaxHeight,
        ),
        "left" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::Left,
        ),
        "right" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::Right,
        ),
        "top" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::Top,
        ),
        "bottom" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::Bottom,
        ),
        "margin" => apply_rect_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::Margin,
        ),
        "margin-left" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::MarginLeft,
        ),
        "margin-right" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::MarginRight,
        ),
        "margin-top" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::MarginTop,
        ),
        "margin-bottom" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::MarginBottom,
        ),
        "padding" => apply_rect_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::Padding,
        ),
        "padding-left" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::PaddingLeft,
        ),
        "padding-right" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::PaddingRight,
        ),
        "padding-top" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::PaddingTop,
        ),
        "padding-bottom" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::PaddingBottom,
        ),
        "border" => apply_border_shorthand(
            name_raw,
            value,
            declarations,
            push_unsupported,
            BorderTarget::All,
        ),
        "border-left" => apply_border_shorthand(
            name_raw,
            value,
            declarations,
            push_unsupported,
            BorderTarget::Left,
        ),
        "border-right" => apply_border_shorthand(
            name_raw,
            value,
            declarations,
            push_unsupported,
            BorderTarget::Right,
        ),
        "border-top" => apply_border_shorthand(
            name_raw,
            value,
            declarations,
            push_unsupported,
            BorderTarget::Top,
        ),
        "border-bottom" => apply_border_shorthand(
            name_raw,
            value,
            declarations,
            push_unsupported,
            BorderTarget::Bottom,
        ),
        "border-width" => apply_border_width(name_raw, value, declarations, push_unsupported),
        "border-left-width" => apply_border_side_width(
            name_raw,
            value,
            declarations,
            push_unsupported,
            BorderTarget::Left,
        ),
        "border-right-width" => apply_border_side_width(
            name_raw,
            value,
            declarations,
            push_unsupported,
            BorderTarget::Right,
        ),
        "border-top-width" => apply_border_side_width(
            name_raw,
            value,
            declarations,
            push_unsupported,
            BorderTarget::Top,
        ),
        "border-bottom-width" => apply_border_side_width(
            name_raw,
            value,
            declarations,
            push_unsupported,
            BorderTarget::Bottom,
        ),
        "border-radius" => apply_border_radius(name_raw, value, declarations, push_unsupported),
        "background-color" => apply_color_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::BackgroundColor,
        ),
        "align-items" => {
            apply_align_items_property(name_raw, value, declarations, push_unsupported)
        }
        "justify-content" => {
            apply_justify_content_property(name_raw, value, declarations, push_unsupported)
        }
        "row-gap" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::RowGap,
        ),
        "column-gap" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::ColumnGap,
        ),
        "gap" => apply_gap(name_raw, value, declarations, push_unsupported),
        "flex-basis" => apply_val_property(
            name_raw,
            value,
            declarations,
            push_unsupported,
            StyleDeclaration::FlexBasis,
        ),
        _ => {
            warn!("unsupported style property '{}'", name_raw);
            push_unsupported(name_raw, value);
        }
    }
}

fn apply_val_property<F>(
    name: &str,
    value: &str,
    declarations: &mut SmallVec<[StyleDeclaration; 8]>,
    push_unsupported: &mut F,
    map: fn(Val) -> StyleDeclaration,
) where
    F: FnMut(&str, &str),
{
    match parse_val(value) {
        Ok(val) => {
            declarations.push(map(val));
        }
        Err(err) => {
            warn!(
                "unsupported style value for '{}': {:?} ({})",
                name, value, err
            );
            push_unsupported(name, value);
        }
    }
}

fn apply_display_property<F>(
    name: &str,
    value: &str,
    declarations: &mut SmallVec<[StyleDeclaration; 8]>,
    push_unsupported: &mut F,
) where
    F: FnMut(&str, &str),
{
    match parse_display(value) {
        Ok(display) => declarations.push(StyleDeclaration::Display(display)),
        Err(err) => {
            warn!(
                "unsupported style value for '{}': {:?} ({})",
                name, value, err
            );
            push_unsupported(name, value);
        }
    }
}

fn apply_align_items_property<F>(
    name: &str,
    value: &str,
    declarations: &mut SmallVec<[StyleDeclaration; 8]>,
    push_unsupported: &mut F,
) where
    F: FnMut(&str, &str),
{
    match parse_align_items(value) {
        Ok(align_items) => declarations.push(StyleDeclaration::AlignItems(align_items)),
        Err(err) => {
            warn!(
                "unsupported style value for '{}': {:?} ({})",
                name, value, err
            );
            push_unsupported(name, value);
        }
    }
}

fn apply_justify_content_property<F>(
    name: &str,
    value: &str,
    declarations: &mut SmallVec<[StyleDeclaration; 8]>,
    push_unsupported: &mut F,
) where
    F: FnMut(&str, &str),
{
    match parse_justify_content(value) {
        Ok(justify_content) => {
            declarations.push(StyleDeclaration::JustifyContent(justify_content));
        }
        Err(err) => {
            warn!(
                "unsupported style value for '{}': {:?} ({})",
                name, value, err
            );
            push_unsupported(name, value);
        }
    }
}

fn apply_rect_property<F>(
    name: &str,
    value: &str,
    declarations: &mut SmallVec<[StyleDeclaration; 8]>,
    push_unsupported: &mut F,
    map: fn(UiRect) -> StyleDeclaration,
) where
    F: FnMut(&str, &str),
{
    match parse_ui_rect(value) {
        Ok(rect) => {
            declarations.push(map(rect));
        }
        Err(err) => {
            warn!(
                "unsupported style value for '{}': {:?} ({})",
                name, value, err
            );
            push_unsupported(name, value);
        }
    }
}

fn apply_color_property<F>(
    name: &str,
    value: &str,
    declarations: &mut SmallVec<[StyleDeclaration; 8]>,
    push_unsupported: &mut F,
    map: fn(Color) -> StyleDeclaration,
) where
    F: FnMut(&str, &str),
{
    match parse_color(value) {
        Ok(color) => {
            declarations.push(map(color));
        }
        Err(err) => {
            warn!(
                "unsupported style value for '{}': {:?} ({})",
                name, value, err
            );
            push_unsupported(name, value);
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum BorderTarget {
    All,
    Left,
    Right,
    Top,
    Bottom,
}

fn apply_border_shorthand<F>(
    name: &str,
    value: &str,
    declarations: &mut SmallVec<[StyleDeclaration; 8]>,
    push_unsupported: &mut F,
    target: BorderTarget,
) where
    F: FnMut(&str, &str),
{
    match parse_border_width_shorthand(value) {
        Ok(parsed) => {
            match target {
                BorderTarget::All => declarations.push(StyleDeclaration::Border(BorderStyle {
                    thickness: UiRect::all(parsed.width),
                })),
                BorderTarget::Left => declarations.push(StyleDeclaration::BorderLeft(parsed.width)),
                BorderTarget::Right => {
                    declarations.push(StyleDeclaration::BorderRight(parsed.width))
                }
                BorderTarget::Top => declarations.push(StyleDeclaration::BorderTop(parsed.width)),
                BorderTarget::Bottom => {
                    declarations.push(StyleDeclaration::BorderBottom(parsed.width))
                }
            }
            if parsed.has_extras {
                warn!("unsupported extra tokens in '{}': {:?}", name, value);
                push_unsupported(name, value);
            }
        }
        Err(err) => {
            warn!(
                "unsupported style value for '{}': {:?} ({})",
                name, value, err
            );
            push_unsupported(name, value);
        }
    }
}

fn apply_border_width<F>(
    name: &str,
    value: &str,
    declarations: &mut SmallVec<[StyleDeclaration; 8]>,
    push_unsupported: &mut F,
) where
    F: FnMut(&str, &str),
{
    match parse_ui_rect(value) {
        Ok(rect) => {
            declarations.push(StyleDeclaration::Border(BorderStyle { thickness: rect }));
        }
        Err(err) => {
            warn!(
                "unsupported style value for '{}': {:?} ({})",
                name, value, err
            );
            push_unsupported(name, value);
        }
    }
}

fn apply_border_side_width<F>(
    name: &str,
    value: &str,
    declarations: &mut SmallVec<[StyleDeclaration; 8]>,
    push_unsupported: &mut F,
    target: BorderTarget,
) where
    F: FnMut(&str, &str),
{
    match parse_val(value) {
        Ok(val) => match target {
            BorderTarget::Left => declarations.push(StyleDeclaration::BorderLeft(val)),
            BorderTarget::Right => declarations.push(StyleDeclaration::BorderRight(val)),
            BorderTarget::Top => declarations.push(StyleDeclaration::BorderTop(val)),
            BorderTarget::Bottom => declarations.push(StyleDeclaration::BorderBottom(val)),
            BorderTarget::All => declarations.push(StyleDeclaration::Border(BorderStyle {
                thickness: UiRect::all(val),
            })),
        },
        Err(err) => {
            warn!(
                "unsupported style value for '{}': {:?} ({})",
                name, value, err
            );
            push_unsupported(name, value);
        }
    }
}

fn apply_border_radius<F>(
    name: &str,
    value: &str,
    declarations: &mut SmallVec<[StyleDeclaration; 8]>,
    push_unsupported: &mut F,
) where
    F: FnMut(&str, &str),
{
    match parse_border_radius(value) {
        Ok(radius) => {
            declarations.push(StyleDeclaration::BorderRadius(radius));
        }
        Err(err) => {
            warn!(
                "unsupported style value for '{}': {:?} ({})",
                name, value, err
            );
            push_unsupported(name, value);
        }
    }
}

fn apply_gap<F>(
    name: &str,
    value: &str,
    declarations: &mut SmallVec<[StyleDeclaration; 8]>,
    push_unsupported: &mut F,
) where
    F: FnMut(&str, &str),
{
    match parse_gap(value) {
        Ok((row, column)) => {
            declarations.push(StyleDeclaration::Gap { row, column });
        }
        Err(err) => {
            warn!(
                "unsupported style value for '{}': {:?} ({})",
                name, value, err
            );
            push_unsupported(name, value);
        }
    }
}

#[derive(Debug)]
enum StyleParseError {
    Empty,
    InvalidNumber,
    InvalidColor(String),
    InvalidKeyword(String),
    UnsupportedUnit(String),
    WrongArity {
        expected: &'static str,
        found: usize,
    },
}

impl std::fmt::Display for StyleParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StyleParseError::Empty => write!(f, "empty value"),
            StyleParseError::InvalidNumber => write!(f, "invalid number"),
            StyleParseError::InvalidColor(value) => {
                write!(f, "invalid color '{}'", value)
            }
            StyleParseError::InvalidKeyword(value) => {
                write!(f, "invalid keyword '{}'", value)
            }
            StyleParseError::UnsupportedUnit(unit) => {
                write!(f, "unsupported unit '{}'", unit)
            }
            StyleParseError::WrongArity { expected, found } => {
                write!(f, "expected {}, found {}", expected, found)
            }
        }
    }
}

fn strip_important(value: &str) -> &str {
    let trimmed = value.trim();
    if let Some(stripped) = trimmed.strip_suffix("!important") {
        stripped.trim_end()
    } else {
        trimmed
    }
}

fn parse_val(value: &str) -> Result<Val, StyleParseError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(StyleParseError::Empty);
    }
    if trimmed.eq_ignore_ascii_case("auto") {
        return Ok(Val::Auto);
    }
    if let Some(number) = trimmed.strip_suffix("px") {
        return Ok(Val::Px(parse_number(number)?));
    }
    if let Some(number) = trimmed.strip_suffix('%') {
        return Ok(Val::Percent(parse_number(number)?));
    }
    if let Some(number) = trimmed.strip_suffix("vw") {
        return Ok(Val::Vw(parse_number(number)?));
    }
    if let Some(number) = trimmed.strip_suffix("vh") {
        return Ok(Val::Vh(parse_number(number)?));
    }
    if let Some(number) = trimmed.strip_suffix("vmin") {
        return Ok(Val::VMin(parse_number(number)?));
    }
    if let Some(number) = trimmed.strip_suffix("vmax") {
        return Ok(Val::VMax(parse_number(number)?));
    }
    if let Ok(number) = trimmed.parse::<f32>() {
        return Ok(Val::Px(number));
    }
    let (number, unit) = split_unit(trimmed);
    if unit.is_empty() {
        return Err(StyleParseError::InvalidNumber);
    }
    if parse_number(number).is_err() {
        return Err(StyleParseError::InvalidNumber);
    }
    Err(StyleParseError::UnsupportedUnit(unit.to_string()))
}

fn parse_color(value: &str) -> Result<Color, StyleParseError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(StyleParseError::Empty);
    }
    let lowered = trimmed.to_ascii_lowercase();
    if lowered == "transparent" {
        return Ok(Color::NONE);
    }
    if let Ok(srgba) = Srgba::hex(trimmed) {
        return Ok(Color::from(srgba));
    }
    let color = match lowered.as_str() {
        "black" => basic::BLACK,
        "silver" => basic::SILVER,
        "gray" | "grey" => basic::GRAY,
        "white" => basic::WHITE,
        "maroon" => basic::MAROON,
        "red" => basic::RED,
        "purple" => basic::PURPLE,
        "fuchsia" => basic::FUCHSIA,
        "green" => basic::GREEN,
        "lime" => basic::LIME,
        "olive" => basic::OLIVE,
        "yellow" => basic::YELLOW,
        "navy" => basic::NAVY,
        "blue" => basic::BLUE,
        "teal" => basic::TEAL,
        "aqua" => basic::AQUA,
        _ => return Err(StyleParseError::InvalidColor(trimmed.to_string())),
    };
    Ok(Color::from(color))
}

fn parse_display(value: &str) -> Result<Display, StyleParseError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(StyleParseError::Empty);
    }
    let lowered = trimmed.to_ascii_lowercase();
    match lowered.as_str() {
        "flex" => Ok(Display::Flex),
        "grid" => Ok(Display::Grid),
        "block" => Ok(Display::Block),
        "none" => Ok(Display::None),
        "inline-flex" => Ok(Display::Flex),
        "inline-grid" => Ok(Display::Grid),
        _ => Err(StyleParseError::InvalidKeyword(trimmed.to_string())),
    }
}

fn parse_align_items(value: &str) -> Result<AlignItems, StyleParseError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(StyleParseError::Empty);
    }
    let lowered = trimmed.to_ascii_lowercase();
    match lowered.as_str() {
        "default" | "normal" | "auto" => Ok(AlignItems::Default),
        "start" => Ok(AlignItems::Start),
        "end" => Ok(AlignItems::End),
        "flex-start" => Ok(AlignItems::FlexStart),
        "flex-end" => Ok(AlignItems::FlexEnd),
        "center" => Ok(AlignItems::Center),
        "baseline" => Ok(AlignItems::Baseline),
        "stretch" => Ok(AlignItems::Stretch),
        _ => Err(StyleParseError::InvalidKeyword(trimmed.to_string())),
    }
}

fn parse_justify_content(value: &str) -> Result<JustifyContent, StyleParseError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(StyleParseError::Empty);
    }
    let lowered = trimmed.to_ascii_lowercase();
    match lowered.as_str() {
        "default" | "normal" | "auto" => Ok(JustifyContent::Default),
        "start" => Ok(JustifyContent::Start),
        "end" => Ok(JustifyContent::End),
        "flex-start" => Ok(JustifyContent::FlexStart),
        "flex-end" => Ok(JustifyContent::FlexEnd),
        "center" => Ok(JustifyContent::Center),
        "stretch" => Ok(JustifyContent::Stretch),
        "space-between" => Ok(JustifyContent::SpaceBetween),
        "space-around" => Ok(JustifyContent::SpaceAround),
        "space-evenly" => Ok(JustifyContent::SpaceEvenly),
        _ => Err(StyleParseError::InvalidKeyword(trimmed.to_string())),
    }
}

fn parse_number(raw: &str) -> Result<f32, StyleParseError> {
    raw.trim()
        .parse::<f32>()
        .map_err(|_| StyleParseError::InvalidNumber)
}

fn split_unit(value: &str) -> (&str, &str) {
    let bytes = value.as_bytes();
    let mut split = value.len();
    while split > 0 {
        let byte = bytes[split - 1];
        if (byte as char).is_ascii_alphabetic() || byte == b'%' {
            split -= 1;
        } else {
            break;
        }
    }
    value.split_at(split)
}

fn parse_val_list(value: &str) -> Result<SmallVec<[Val; 4]>, StyleParseError> {
    let mut values = SmallVec::new();
    for token in value.split_whitespace() {
        values.push(parse_val(token)?);
    }
    if values.is_empty() {
        return Err(StyleParseError::Empty);
    }
    if values.len() > 4 {
        return Err(StyleParseError::WrongArity {
            expected: "1-4 values",
            found: values.len(),
        });
    }
    Ok(values)
}

fn parse_ui_rect(value: &str) -> Result<UiRect, StyleParseError> {
    let values = parse_val_list(value)?;
    let rect = match values.as_slice() {
        [all] => UiRect::all(*all),
        [vertical, horizontal] => UiRect::new(*horizontal, *horizontal, *vertical, *vertical),
        [top, horizontal, bottom] => UiRect::new(*horizontal, *horizontal, *top, *bottom),
        [top, right, bottom, left] => UiRect::new(*left, *right, *top, *bottom),
        _ => {
            return Err(StyleParseError::WrongArity {
                expected: "1-4 values",
                found: values.len(),
            });
        }
    };
    Ok(rect)
}

fn parse_border_radius(value: &str) -> Result<BorderRadius, StyleParseError> {
    let value = value.split_once('/').map(|(left, _)| left).unwrap_or(value);
    let values = parse_val_list(value)?;
    let radius = match values.as_slice() {
        [all] => BorderRadius::all(*all),
        [first, second] => BorderRadius::new(*first, *second, *first, *second),
        [first, second, third] => BorderRadius::new(*first, *second, *third, *second),
        [first, second, third, fourth] => BorderRadius::new(*first, *second, *third, *fourth),
        _ => {
            return Err(StyleParseError::WrongArity {
                expected: "1-4 values",
                found: values.len(),
            });
        }
    };
    Ok(radius)
}

fn parse_gap(value: &str) -> Result<(Val, Val), StyleParseError> {
    let values = parse_val_list(value)?;
    match values.as_slice() {
        [all] => Ok((*all, *all)),
        [row, column] => Ok((*row, *column)),
        _ => Err(StyleParseError::WrongArity {
            expected: "1-2 values",
            found: values.len(),
        }),
    }
}

struct BorderWidthParse {
    width: Val,
    has_extras: bool,
}

fn parse_border_width_shorthand(value: &str) -> Result<BorderWidthParse, StyleParseError> {
    let mut width = None;
    let mut has_extras = false;
    let mut unsupported_unit = None;
    for token in value.split_whitespace() {
        match parse_val(token) {
            Ok(val) => {
                if width.is_none() {
                    width = Some(val);
                } else {
                    has_extras = true;
                }
            }
            Err(StyleParseError::UnsupportedUnit(unit)) => {
                has_extras = true;
                unsupported_unit = Some(unit);
            }
            Err(_) => {
                has_extras = true;
            }
        }
    }
    let Some(width) = width else {
        if let Some(unit) = unsupported_unit {
            return Err(StyleParseError::UnsupportedUnit(unit));
        }
        return Err(StyleParseError::InvalidNumber);
    };
    Ok(BorderWidthParse { width, has_extras })
}
