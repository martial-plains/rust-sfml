#![warn(clippy::pedantic, clippy::nursery)]

use std::{
    env,
    error::Error,
    fs::{create_dir_all, File},
    io::{copy, Write},
    path::{Path, PathBuf},
};

use bindgen::CargoCallbacks;
use flate2::read::GzDecoder;
use reqwest::get;
use tar::Archive;
use tempfile::Builder;
use zip::ZipArchive;

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=CSFML");

    // Read feature flags
    let feat_audio = env::var("CARGO_FEATURE_AUDIO").is_ok();
    let feat_window = env::var("CARGO_FEATURE_WINDOW").is_ok();
    let feat_graphics = env::var("CARGO_FEATURE_GRAPHICS").is_ok();

    // If the CSFML directory doesn't exist, download and extract it
    if !Path::new("CSFML").exists() {
        let url = get_cfml_url();
        let _ = download_and_extract_csfml(url).await.unwrap();
    }

    // Set the library search path
    println!("cargo:rustc-link-search=/sys/CSFML/lib");

    // Generate wrapper header and bindings
    let bindings_header = "wrapper.h";
    generate_wrapper(bindings_header, feat_audio, feat_window, feat_graphics);
    generate_bindings(bindings_header);
}

/// Downloads and extracts the CSFML archive (ZIP or tar.gz) based on the platform.
async fn download_and_extract_csfml(url: &str) -> Result<PathBuf, Box<dyn Error>> {
    let archive = download_file(url).await?;
    let extracted_dir = Path::new("CSFML");

    // Determine if it's a zip or tar.gz and extract accordingly
    if std::path::Path::new(url)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("zip"))
    {
        extract_zip(archive, extracted_dir)?;
    } else if url.ends_with(".tar.gz") {
        extract_tar_gz(archive, extracted_dir)?;
    } else {
        return Err("Unsupported archive format".into());
    }

    Ok(extracted_dir.to_path_buf())
}

/// Downloads the file at the specified URL and returns a file handle.
async fn download_file(url: &str) -> Result<File, Box<dyn Error>> {
    let tmp_dir = Builder::new().prefix("cfml").tempdir()?;
    let response = get(url).await?;
    let fname = response
        .url()
        .path_segments()
        .and_then(std::iter::Iterator::last)
        .unwrap_or_default()
        .to_string();

    let path = tmp_dir.path().join(fname);
    let mut writter = File::create(&path)?;
    let reader = File::open(&path)?;
    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut writter)?;

    Ok(reader)
}

/// Extracts a ZIP archive into the given directory.
fn extract_zip(archive: File, extracted_dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut zip = ZipArchive::new(archive)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let outpath = extracted_dir.join(file.mangled_name());
        if file.name().ends_with('/') {
            create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

/// Extracts a tar.gz archive into the given directory.
fn extract_tar_gz(archive: File, extracted_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let tar = GzDecoder::new(archive);

    // Create the archive and extract it
    let _ = Archive::new(tar)
        .entries()?
        .filter_map(std::result::Result::ok)
        .map(|mut entry| -> Result<(), Box<dyn Error>> {
            let path = entry.path()?;
            let new_path = adjust_path_for_csfml(path.to_path_buf())?;
            entry.unpack(new_path)?;
            Ok(())
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    Ok(extracted_dir.to_path_buf())
}

/// Adjusts the extracted path to ensure that CSFML is extracted into the correct directory.
fn adjust_path_for_csfml(path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let top_level_folder = path
        .components()
        .next()
        .map(|component| component.as_os_str().to_owned())
        .unwrap_or_default();

    let new_path = if top_level_folder.is_empty() {
        path
    } else {
        Path::new("CSFML").join(path.strip_prefix(&top_level_folder)?)
    };

    Ok(new_path)
}

/// Returns the appropriate download URL for CSFML based on the target platform.
fn get_cfml_url() -> &'static str {
    if is_aarch64_apple_darwin() {
        return "https://www.sfml-dev.org/files/CSFML-2.6.1-macOS-clang-arm64.tar.gz";
    }
    if is_x86_64_apple_darwin() {
        return "https://www.sfml-dev.org/files/CSFML-2.6.1-macOS-clang-64-bit.tar.gz";
    }
    if is_x86_64_pc_windows_msvc() {
        return "https://www.sfml-dev.org/files/CSFML-2.6.1-windows-64-bit.zip";
    }
    if is_i686_pc_windows_msvc() {
        return "https://www.sfml-dev.org/files/CSFML-2.6.1-windows-32-bit.zip";
    }
    panic!("No precompiled CSFML found for this system.");
}

/// Generates the wrapper header file based on the selected features.
fn generate_wrapper(
    bindings_header: &str,
    feat_audio: bool,
    feat_window: bool,
    feat_graphics: bool,
) {
    let mut file = File::create(bindings_header).unwrap();
    let mut headers = Vec::new();

    headers.push("SFML/System.h");
    link_sfml_subsystem("system");

    if feat_audio {
        headers.push("SFML/Audio.h");
        link_sfml_subsystem("audio");
    }

    if feat_window {
        headers.push("SFML/Window.h");
        link_sfml_subsystem("window");
    }

    if feat_graphics {
        headers.push("SFML/Graphics.h");
        link_sfml_subsystem("graphics");
    }

    for header in headers {
        writeln!(file, "#include <{header}>").unwrap();
    }
}

/// Generates the bindings using the specified wrapper header.
fn generate_bindings(binding_header: &str) {
    let bindings = bindgen::Builder::default()
        .clang_arg("-I./CSFML/include")
        .header(binding_header)
        .parse_callbacks(Box::new(CargoCallbacks::new()))
        .use_core()
        .derive_default(true)
        .derive_copy(true)
        .derive_debug(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .derive_partialeq(true)
        .derive_partialord(true)
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: true,
            is_global: true,
        })
        .prepend_enum_name(false)
        .generate_cstr(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

/// Links the appropriate SFML subsystem library.
fn link_sfml_subsystem(name: &str) {
    println!("cargo:rustc-link-lib=dylib=csfml-{name}");
}

/// Platform check functions
fn is_aarch64_apple_darwin() -> bool {
    is_target("aarch64-apple-darwin")
}

fn is_x86_64_apple_darwin() -> bool {
    is_target("x86_64-apple-darwin")
}

fn is_x86_64_pc_windows_msvc() -> bool {
    is_target("x86_64-pc-windows-msvc")
}

fn is_i686_pc_windows_msvc() -> bool {
    is_target("i686-pc-windows-msvc")
}

fn is_target(triple: &str) -> bool {
    env::var("TARGET").unwrap_or_default() == triple
}
