mod settings;
mod menu;
mod progressbar;
use std::{path::{Path, PathBuf}, time::Duration};
use menu::Menu;
use progressbar::progressbar;
use settings::Settings;


fn main()
{
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

fn rename(settings: Settings) 
{
    let menu = menu::select_rename_menu();
    if let Some(dirs) = utilites::io::get_only_dirs(&settings.target_dir)
    {
        if let Menu::Skip = menu 
        {
            copy(&settings, &dirs);
        }
        else 
        {
            let pb = progressbar(dirs.len() as u64);
            for dir in &dirs
            {
                if let Some(dir_name) = dir.file_name().and_then(|n| n.to_str())
                {
                    let full_path = Path::new(&settings.target_dir).join(dir_name);
                    let new_name = match menu
                    {
                        Menu::ToGis => settings.get_second_name_by_first_name(dir_name),
                        Menu::FromGis => settings.get_first_name_by_second_name(dir_name),
                        Menu::Skip => break,
                    };
                    if let Some(nn) = new_name
                    {
                        let new_name_full_path = Path::new(&settings.target_dir).join(nn);
                        pb.set_message([dir_name, "-> ", nn].concat());
                        let _ = std::fs::rename(full_path, new_name_full_path);
                    }
                    pb.inc(1);
                }   
            }
            pb.finish_with_message("переименование завершено");
            std::thread::sleep(Duration::from_millis(2000));
            pb.finish_and_clear();
            copy(&settings, &dirs);
        }
        std::thread::sleep(Duration::from_millis(3000));
    }
}

fn copy(settings: &Settings, dirs: &Vec<PathBuf>)
{
    if settings.copy_targets.len() > 0
    {
        match menu::copy_menu(&settings)
        {
            menu::CopyMenu::Copy(path) =>
            {
                let path: PathBuf = path.into();
                if std::fs::exists(&path).is_ok_and(|f| f == true)
                {
                    let pb = progressbar(dirs.len() as u64);
                    let mut has_errors = false;
                    for d in dirs
                    {
                        if let Some(name) = d.file_name().and_then(|f| f.to_str())
                        {
                            if settings.get_second_name_by_first_name(name).is_some() || settings.get_first_name_by_second_name(name).is_some()
                            {
                                let target_path = Path::new(&path).join(name);
                                let cr = utilites::io::copy_dir_all(d, &target_path);
                                if cr.is_err()
                                {
                                    logger::error!("Ошибка копирования {} -> {}", d.display(), cr.err().unwrap());
                                    has_errors = true;
                                }
                                else 
                                {
                                    pb.inc(1);    
                                }
                            }
                        }
                    }
                    if has_errors
                    {
                        pb.finish_with_message("Копирование завершено с ошибками");
                    }
                    else 
                    {
                        pb.finish_with_message("Копирование завершено");  
                    }
                }
                else 
                {
                    logger::error!("Директория назначения {} не существует", &path.display());
                    std::thread::sleep(Duration::from_millis(3000));
                    exit();
                }
            },
            menu::CopyMenu::Exit => exit(),
        }
    }
}