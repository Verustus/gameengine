use std::{fs::{read_dir, File, DirEntry}, io::Write};
use spirv_compiler::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut compiler = CompilerBuilder::new().with_include_dir("shaders/include").with_source_language(SourceLanguage::GLSL).with_target_spirv(SpirvVersion::V1_6).build().unwrap();
    for path in read_dir("shaders").unwrap() {
        let shader: DirEntry = path.unwrap();
        if shader.metadata().unwrap().is_file() {
            let compiled_data: Vec<u32> = compiler.compile_from_string(std::fs::read_to_string(shader.path()).unwrap().as_str(), get_shader_kind(shader.file_name().to_str().unwrap()).unwrap()).unwrap();
            let mut compiled_shader = File::create("shaderCache/".to_owned()+shader.file_name().as_os_str().to_str().unwrap()+".spv").unwrap();
            for &value in &compiled_data {
                compiled_shader.write_all(&value.to_ne_bytes()).unwrap();
            }
        }
    }
    Ok(())
}

fn get_shader_kind(file_name: &str) -> Option<ShaderKind> {
    match file_name.split('.').last().unwrap() {
        "vs" => Some(ShaderKind::Vertex),
        "fs" => Some(ShaderKind::Fragment),
        "gs" => Some(ShaderKind::Geometry),
        "tcs" => Some(ShaderKind::TessControl),
        "tes" => Some(ShaderKind::TessEvaluation),
        _ => { assert!(false, "The shader type is not supported or the filename is incorrect!"); None }
    }
}