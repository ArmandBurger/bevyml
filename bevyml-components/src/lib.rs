use bevy_ecs::component::Component;
use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};

macro_rules! element {
    ($name:ident) => {
        #[derive(Reflect, Component, Clone, Copy, Debug, Default, Serialize, Deserialize)]
        pub struct $name;
    };
}

element!(HtmlElement);
element!(HeadElement);
element!(BodyElement);
element!(TitleElement);
element!(MetaElement);
element!(LinkElement);
element!(StyleElement);
element!(ScriptElement);
element!(DivElement);
element!(SpanElement);
element!(PElement);
element!(AElement);
element!(ImgElement);
element!(ButtonElement);
element!(InputElement);
element!(LabelElement);
element!(TextareaElement);
element!(SelectElement);
element!(OptionElement);
element!(UlElement);
element!(OlElement);
element!(LiElement);
element!(TableElement);
element!(TheadElement);
element!(TbodyElement);
element!(TfootElement);
element!(TrElement);
element!(ThElement);
element!(TdElement);
element!(HeaderElement);
element!(FooterElement);
element!(NavElement);
element!(MainElement);
element!(SectionElement);
element!(ArticleElement);
element!(AsideElement);
element!(FormElement);
element!(CanvasElement);
element!(SvgElement);
element!(BrElement);
element!(HrElement);

element!(H1Element);
element!(H2Element);
element!(H3Element);
element!(H4Element);
element!(H5Element);
element!(H6Element);
