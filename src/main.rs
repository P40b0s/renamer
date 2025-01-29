mod settings;
mod menu;
mod progressbar;
mod zipper;
use std::{fs::File, io::Write, path::{Path, PathBuf}, time::Duration};
use menu::{CopyMenu, Menu};
use progressbar::progressbar;
use settings::{Mapping, SearchResult, Settings};



fn main()
{
    let _  = logger::StructLogger::new_default();
    load_config();
}

fn load_config()
{
    if let Some(settings) = settings::Settings::load_settings()
    {
        copy(settings);
    }
    else 
    {
        match menu::reload_config_menu()
        {
            menu::ConfigMenu::Reload => load_config(),
            menu::ConfigMenu::Exit => exit(),
        }

    }
}

fn exit()
{
    if cfg!(windows) 
    {
        std::process::exit(0x0100);
    } 
    else
    {
        std::process::exit(0);
    }
}

fn get_source_dirs(settings: &Settings) -> Option<Vec<SearchResult>>
{
    if let Some(dirs) = utilites::io::get_only_dirs(&settings.source_directory)
    {
        let mut result: Vec<SearchResult> = Vec::with_capacity(dirs.len());
        for path in dirs.into_iter()
        {
            if let Some(name) = path.file_name()
            {
                if let Some(name) = name.to_str()
                {
                    if let Some(map) = settings.get_mapping(name)
                    {
                        result.push(
                            SearchResult 
                            { 
                                packet_source_path: path.clone(),
                                packet_dir_name: name.to_owned(),
                                map
                            }
                        );
                    }
                }
            }
        }
        Some(result)
    }
    else 
    {
        None
    }
}

fn copy(settings: Settings) 
{
    let mut errors = Vec::new();
    if let Some(dirs) = get_source_dirs(&settings)
    {
        let selected_menu = menu::copy_menu(&settings,  &dirs);
        let pb = progressbar(selected_menu.len() as u64);
        let mut has_errors = false;
        for  menu in selected_menu
        {
           match menu
           {
                CopyMenu::Copy(val) =>
                {
                    let source_path = &val.packet_source_path;
                    let target_path = Path::new(&settings.target_directory);
                    let target_dir = &val.map.borrow().dir_name;
                    pb.set_prefix(target_dir.to_owned());
                    let compressed = zipper::zip_packet(&pb, target_dir, source_path, target_path);
                    if compressed.is_err()
                    {
                        let error = format!("🔴 Ошибка архивирования {} в {} -> {}", source_path.display(), target_path.display(), compressed.err().unwrap());
                        pb.println(&error);
                        errors.push(error);
                        has_errors = true;
                    }
                    else 
                    {
                        pb.println(&[target_dir, " ✅"].concat());
                        pb.inc(1);
                    }
                },
                _ => ()
           };
        }
        if has_errors
        {
            pb.finish_with_message("Архивирование завершено с ошибками");
            for e in errors
            {
                logger::error!("{}", e);
            }
        }
        else 
        {
            pb.finish_with_message("Архивирование завершено");
        }
        std::thread::sleep(Duration::from_millis(3000)); 
        
    }
}

