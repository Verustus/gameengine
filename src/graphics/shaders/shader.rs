pub enum ShaderType {
    Vertex,
    Fragment,
    Tesselation,
    Geometry
}

pub struct Shader {
    pub shader_type: ShaderType,
    pub shader_binary: Vec<u32>
}