use shaderc;
use shaderc::ShaderKind;

use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

struct ShaderFile {
    shader_kind: ShaderKind,
    path: PathBuf,
}

impl ShaderFile {
    fn new(shader_kind: ShaderKind, path: PathBuf) -> Self {
        Self { shader_kind, path }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // shaderc requires environment variable SHADERC_LIB_DIR=C:/Program Files (x86)/shaderc/lib/
    // to be set.  Do not use quotes (yes, there is a space).  Do use forward-slash.

    // Tell the build script to only run again if we change our source shaders
    println!("cargo:rerun-if-changed=shaders");

    let in_dir = Path::new("shaders");

    let in_shaders = [
        ShaderFile::new(ShaderKind::Vertex, in_dir.join("simple.vert")),
        ShaderFile::new(ShaderKind::Fragment, in_dir.join("simple.frag")),
    ];

    let out_dir_name = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir_name);

    let mut compiler = shaderc::Compiler::new().unwrap();

    for in_shader in &in_shaders {
        println!(
            "cargo:rerun-if-changed={}",
            in_shader.path.to_string_lossy()
        );

        let in_file_name = in_shader.path.file_name().unwrap().to_string_lossy();

        let source = fs::read_to_string(&in_shader.path)?;

        let compiled_bytes = compiler
            .compile_into_spirv(&source, in_shader.shader_kind, &in_file_name, "main", None)
            .unwrap();

        let out_path = out_dir.join(format!("{}.spv", &in_file_name));

        fs::write(&out_path, compiled_bytes.as_binary_u8())?;
    }

    Ok(())
}
