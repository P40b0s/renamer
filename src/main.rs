mod settings;
mod menu;
mod progressbar;
use std::{path::{Path, PathBuf}, time::Duration};
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
        rename(settings);
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
///(PathBuf, String) (полный путь)
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
        // Some(
        //     dirs
        //     .into_iter()
        //     .filter_map(|f| f.file_name().as_ref()
        //     .and_then(|os| os.to_str()
        //     .and_then(|ext| settings.get_mapping(ext)
        //     .and_then(|_| Some((f,ext.to_owned()))))))
        //     .collect()
        // )
        
    }
    else 
    {
        None
    }
}

fn rename(settings: Settings) 
{
   
    if let Some(dirs) = get_source_dirs(&settings)
    {
        //FIXME одновременно выбраны пакеты и меню выход
        let selected_menu = menu::copy_menu(&settings,  &dirs);
        let pb = progressbar(selected_menu.len() as u64);
        for  menu in selected_menu
        {
           match menu
           {
                CopyMenu::Copy(val) =>
                {
                    let source_path = &val.packet_source_path;
                    let target_path = Path::new(&settings.target_directory).join(&val.map.borrow().dir_name);
                    let mut has_errors = false;
                    let cr = utilites::io::copy_dir_all(source_path, &target_path);
                    if cr.is_err()
                    {
                        logger::error!("Ошибка копирования {} в {} -> {}", source_path.display(), &target_path.display(), cr.err().unwrap());
                        has_errors = true;
                    }
                    else 
                    {
                        pb.inc(1);    
                    }
                    if has_errors
                    {
                        pb.finish_with_message("Копирование завершено с ошибками");
                    }
                    else 
                    {
                        pb.finish_with_message("Копирование завершено");  
                    }
                },
                _ => ()
           };
        }
        std::thread::sleep(Duration::from_millis(3000));
    }
}

// fn copy(settings: &Settings, dirs: &Vec<PathBuf>)
// {
//     if settings.copy_targets.len() > 0
//     {
//         match menu::copy_menu(&settings)
//         {
//             menu::CopyMenu::Copy(path) =>
//             {
//                 let path: PathBuf = path.into();
//                 if std::fs::exists(&path).is_ok_and(|f| f == true)
//                 {
//                     let pb = progressbar(dirs.len() as u64);
//                     let mut has_errors = false;
//                     for d in dirs
//                     {
//                         if let Some(name) = d.file_name().and_then(|f| f.to_str())
//                         {
//                             if true //settings.get_second_name_by_first_name(name).is_some() || settings.get_first_name_by_second_name(name).is_some()
//                             {
//                                 let target_path = Path::new(&path).join(name);
//                                 let cr = utilites::io::copy_dir_all(d, &target_path);
//                                 if cr.is_err()
//                                 {
//                                     logger::error!("Ошибка копирования {} -> {}", d.display(), cr.err().unwrap());
//                                     has_errors = true;
//                                 }
//                                 else 
//                                 {
//                                     pb.inc(1);    
//                                 }
//                             }
//                         }
//                     }
//                     if has_errors
//                     {
//                         pb.finish_with_message("Копирование завершено с ошибками");
//                     }
//                     else 
//                     {
//                         pb.finish_with_message("Копирование завершено");  
//                     }
//                 }
//                 else 
//                 {
//                     logger::error!("Директория назначения {} не существует", &path.display());
//                     std::thread::sleep(Duration::from_millis(3000));
//                     exit();
//                 }
//             },
//             menu::CopyMenu::Exit => exit(),
//         }
//     }
// }