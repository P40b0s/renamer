use std::fmt::Display;

use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::settings::Settings;

pub fn select_rename_menu() -> Menu
{
    let mut selections = vec![
        Menu::ToGis,
        Menu::FromGis,
        Menu::Skip
    ];

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Выбор направления переименования")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    let item = selections.swap_remove(selection);
    item
    //println!("Enjoy your {}!", selections[selection]);
}
#[derive(Debug)]
pub enum Menu
{
    ///3.1 наменования в gis наимеонвания
    ToGis,
    ///gis наимеонвания в 3.1 наменования 
    FromGis,
    ///Пропустить процесс переименования
    Skip
}
impl Display for Menu
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self
        {
            Menu::ToGis => f.write_str("3.1 в ГИС"),
            Menu::FromGis => f.write_str("ГИС в 3.1"),
            Menu::Skip => f.write_str("Пропустить")
        }
    }
}

pub fn reload_config_menu() -> ConfigMenu
{
    let mut selections = vec![
        ConfigMenu::Reload,
        ConfigMenu::Exit
    ];

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Ошибка загрузки файла конфигурации")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    let item = selections.swap_remove(selection);
    item
    //println!("Enjoy your {}!", selections[selection]);
}


#[derive(Debug)]
pub enum ConfigMenu
{
    Reload,
    Exit
}
impl Display for ConfigMenu
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self
        {
            ConfigMenu::Reload => f.write_str("Перезагрузить файл конфигурации"),
            ConfigMenu::Exit => f.write_str("Выход")
        }
    }
}




#[derive(Debug)]
pub enum CopyMenu
{
    Copy(String),
    Exit
}
impl Display for CopyMenu
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self
        {
            CopyMenu::Copy(c) => f.write_str(&["Копировать в ", c].concat()),
            CopyMenu::Exit => f.write_str("Выход")
        }
    }
}


pub fn copy_menu(settings: &Settings) -> CopyMenu
{
    let mut selections: Vec<CopyMenu> = settings.copy_targets.iter().map(|m| CopyMenu::Copy(m.clone())).collect();
    selections.push(CopyMenu::Exit);
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Выбор директории для копирования пакетов")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    let item = selections.swap_remove(selection);
    item
}