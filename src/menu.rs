use std::fmt::Display;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, MultiSelect};
use crate::settings::{SearchResult, Settings};

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
pub enum CopyMenu<'a>
{
    Copy(&'a SearchResult<'a>),
    SelectAll,
    DeselectAll
}
impl<'a> Display for CopyMenu<'a>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self
        {
            CopyMenu::Copy(m) => 
            {
                if let Some(base_name) = m.map.borrow().bank_name.as_ref()
                {
                    f.write_str(&[&m.packet_dir_name, " (", &base_name, ")"].concat())    
                }
                else 
                {
                    f.write_str(&m.packet_dir_name)    
                }
            }

            CopyMenu::SelectAll => f.write_str("Выбрать все"),
            CopyMenu::DeselectAll => f.write_str("Убрать все"),
        }
    }
}



pub fn copy_menu<'a>(settings: &Settings, source_dirs: &'a Vec<SearchResult>) -> Vec<CopyMenu<'a>>
{

    let mut multiselected: Vec<CopyMenu> = source_dirs.iter().map(|m|
    {
        CopyMenu::Copy(m)
    }).collect();
    let selected: Vec<bool> = source_dirs.iter().map(|m| m.map.borrow().selected).collect();
    multiselected.push(CopyMenu::SelectAll);
    multiselected.push(CopyMenu::DeselectAll);
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Выберите пакеты для копирования")
        .items(&multiselected[..])
        .defaults(&selected)
        .interact()
        .unwrap();

    if selections.is_empty() 
    {
        return Vec::new();
    }
    else if selections.contains(&(multiselected.len() -1))
    {
        settings.map.values().for_each(|f| f.borrow_mut().selected = false);
        return  copy_menu(&settings, source_dirs);
    }
    else if selections.contains(&(multiselected.len() -2))
    {
        settings.map.values().for_each(|f| f.borrow_mut().selected = true);
        return  copy_menu(&settings, source_dirs);
    }
    let selected_items = multiselected
    .into_iter()
    .enumerate()
    .filter(move |f| selections.contains(&f.0))
    .map(|m| 
    {
        if let CopyMenu::Copy(cp) = m.1
        {
            cp.map.borrow_mut().selected = true;
        }
        m.1
    })
    .collect();
    settings.save();
    selected_items
    
}