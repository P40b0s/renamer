use std::{fs::File, io::Write, path::Path};
use zip::{write::{FileOptions, SimpleFileOptions}, ZipWriter};

///`progress_bar` для оповещения юзера
/// `zip_root_dir` директория пакета который архивируется
/// 
fn zip_recursive(progress_bar: &indicatif::ProgressBar, packet_name: &str, zip_root_dir: &str,  zip: &mut ZipWriter<File>, src: impl AsRef<Path>, dst: impl AsRef<Path>) -> zip::result::ZipResult<()>
{
    let src = src.as_ref();
    for entry in std::fs::read_dir(src)? 
    {
        let entry = entry?;
        let name = entry.path();
        let name = name.to_str().unwrap();
        if let Some(relative_path) = name.split_once(&[zip_root_dir, std::path::MAIN_SEPARATOR_STR].concat())
        {
            //logger::info!("relative_path: {:?}, root_dir {}", relative_path, zip_root_dir);
            let ty = entry.file_type()?;
            if ty.is_dir() 
            {
                let dir_path = [packet_name, std::path::MAIN_SEPARATOR_STR, relative_path.1, std::path::MAIN_SEPARATOR_STR].concat();
                //logger::info!("dir: {}", relative_dir_path,);
                zip.add_directory(&dir_path, compression_stored())?;
                zip_recursive(progress_bar, packet_name, zip_root_dir, zip, entry.path(), dst.as_ref().join(entry.file_name()))?;
                
            } 
            else 
            {
                //logger::info!("file: {}", relative_path.1);
                let file_path = [packet_name, std::path::MAIN_SEPARATOR_STR, relative_path.1].concat();
                let options = zip_options(&file_path);
                progress_bar.set_message(["архивация -> ", relative_path.1].concat());
                zip.start_file(file_path, options)?;
                let file = utilites::io::read_file_to_binary(entry.path())?;
                zip.write(&file)?;
            }
        }
        else 
        {
            return Err(zip::result::ZipError::InvalidArchive("Ошибка, определения корневой директории пакета"))
        }
    }
    Ok(())
}

pub fn zip_packet(progress_bar: &indicatif::ProgressBar, packet_name: &str, src: impl AsRef<Path>, dst: impl AsRef<Path>) -> zip::result::ZipResult<()>
{
    let trg = dst.as_ref();
    let src = src.as_ref();
    if let Some(zip_root_dir) = src.iter().last().and_then(|l| l.to_str())
    {
        let target_file = Path::new(dst.as_ref()).join([packet_name, ".zip"].concat());
        let file = std::fs::File::create(target_file).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        zip_recursive(progress_bar, packet_name, zip_root_dir, &mut zip,  src, trg)?;
        zip.finish()?;
    }
    else 
    {
       return Err(zip::result::ZipError::InvalidArchive("Ошибка, не обнаружена корневая директория пакета"))
    }
    Ok(())
}
fn zip_options<'a>(zip_path: &str) -> FileOptions<'a, ()>
{
    if let Some(splitted) = zip_path.split_once(".")
    {
        match splitted.1
        {
            "txt" => compression(),
            "doc" => compression(),
            "docx" => compression(),
            "xls" => compression(),
            "xslx" => compression(),
            _ => compression_stored()
        }
    }
    else 
    {
        compression_stored()
    }
}

fn compression_stored<'a>() -> FileOptions<'a, ()>
{
    SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Stored)
    .unix_permissions(0o755)
}
fn compression<'a>() -> FileOptions<'a, ()>
{
    SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Bzip2)
    .unix_permissions(0o755)
}

#[cfg(test)]
mod tests
{
    use std::path::Path;
    use crate::progressbar;

    #[test]
    fn test_recursive()
    {
        let _ = logger::StructLogger::new_default();
        let pb = progressbar(1);
        let _ = super::zip_packet(&pb, "r013100000", Path::new("/home/phobos/projects/rust/renemaer_data/r013100"), Path::new("/home/phobos/projects/rust/renemaer_data/target_1"));

    }
}