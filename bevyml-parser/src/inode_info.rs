use bevy_math::USizeVec2;

#[derive(Debug)]
pub struct INodeInfo<'source> {
    pub kind: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_position: USizeVec2,
    pub end_position: USizeVec2,
    pub text: String,
    pub original_text: &'source str,
}
