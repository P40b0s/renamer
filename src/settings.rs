use std::{cell::RefCell, path::PathBuf};
use serde::{Serialize, Deserialize};

const CONF_FILE_NAME: &str = "config.toml";
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings
{
    pub target_directory : PathBuf,
    pub source_directory: PathBuf,
    pub map : hashbrown::HashMap<String, RefCell<Mapping>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Mapping
{
    pub dir_name: String,
    pub bank_name: Option<String>,
    pub selected: bool
}

#[derive(Clone, Debug)]
pub struct SearchResult<'a>
{
    pub packet_source_path: PathBuf,
    pub packet_dir_name: String,
    pub map: &'a RefCell<Mapping>
}

impl<'a> Settings
{
    pub fn load_settings() -> Option<Self>
    {
        let settings = utilites::deserialize(CONF_FILE_NAME, false, utilites::Serializer::Toml);
        if let Ok(s) = settings
        {
            Some(s)
        }
        else 
        {
            logger::error!("Ошибка загрузки файла конфигурации {} -> {}", CONF_FILE_NAME, settings.err().unwrap());
            None 
        }
    }
    pub fn get_mapping(&self, source_dir_name: &str) -> Option<&RefCell<Mapping>>
    {
        self.map.get(source_dir_name)
    }
    pub fn save(&self)
    {
        let _ = utilites::serialize(self, CONF_FILE_NAME, false, utilites::Serializer::Toml);
    }
}

#[cfg(test)]
mod tests
{
    use std::cell::RefCell;

    use super::{Mapping, Settings};
    #[test]
    fn testsave()
    {
        let mut map = hashbrown::HashMap::new();
        map.insert("r013100".into(), RefCell::new(Mapping 
        { 
            dir_name: "r013100000".into(),
            bank_name: Some("Банк бла бла бла".into()),
            selected: false
        }));
        map.insert("r013200".into(), RefCell::new(Mapping 
        { 
            dir_name: "r013200000".into(),
            bank_name: None,
            selected: false
        }));

        let s = Settings
        {
            target_directory: "C:\\files".into(),
            source_directory: "C:\\files".into(),
            map
            // names_map: hashbrown::HashMap::
            //     ("r013100".into(),"r013100000".into()),
            //     ("r013200".into(),"r013200000".into()),
            //     ("r013300".into(),"r013300000".into()),
            //     ("r013600".into(),"r013600000".into()),
            //     ("r013700".into(),"r013700000".into()),
            //     ("r014000".into(),"r014000000".into()),
            //     ("r014400".into(),"r014400000".into()),
            //     ("r014800".into(),"r014800000".into()),
            //     ("r015000".into(),"r015000000".into()),
            //     ("r015700".into(),"r015700000".into()),
            //     ("r016200".into(),"r016200000".into()),
            //     ("r016700".into(),"r016700000".into()),
            //     ("r016800".into(),"r016800000".into()),
            //     ("r016900".into(),"r016900000".into()),
            //     ("r017100".into(),"r017100000".into()),
            //     ("r017600".into(),"r017600000".into()),
            //     ("r021000".into(),"r021000000".into()),
            //     ("r021100".into(),"r021100000".into()),
            //     ("r022900".into(),"r022900000".into()),
            //     ("r022902".into(),"r028300000".into()),
            //     ("r023500".into(),"r023500000".into()),
            //     ("r023900".into(),"r023900000".into()),
            //     ("r024700".into(),"r024700000".into()),
            //     ("r025100".into(),"r025100000".into()),
            //     ("r027800".into(),"r027800000".into()),
            //     ("r030100".into(),"r030100000".into()),
            //     ("r030800".into(),"r030800000".into()),
            //     ("r030820".into(),"r038200000".into()),
            //     ("r033000".into(),"r033000000".into()),
            //     ("r033400".into(),"r033400000".into()),
            //     ("r036100".into(),"r036100000".into()),
            //     ("r040300".into(),"r040300000".into()),
            //     ("r041200".into(),"r041200000".into()),
            //     ("r041300".into(),"r041300000".into()),
            //     ("r041600".into(),"r041600000".into()),
            //     ("r041800".into(),"r041800000".into()),
            //     ("r042100".into(),"r042100000".into()),
            //     ("r044300".into(),"r044300000".into()),
            //     ("r045200".into(),"r045200000".into()),
            //     ("r046300".into(),"r046300000".into()),
            //     ("r046400".into(),"r046400000".into()),
            //     ("r047300".into(),"r047300000".into()),
            //     ("r054500".into(),"r054500000".into()),
            //     ("r057200".into(),"r057200000".into()),
            //     ("r057400".into(),"r057400000".into()),
            //     ("r058900".into(),"r058900000".into()),
            //     ("r060200".into(),"r060200000".into()),
            //     ("r061700".into(),"r061700000".into()),
            //     ("r061900".into(),"r061900000".into()),
            //     ("r062200".into(),"r062200000".into()),
            //     ("r062400".into(),"r062400000".into()),
            //     ("r063800".into(),"r063800000".into()),
            //     ("r064200".into(),"r064200000".into()),
            //     ("r065400".into(),"r065400000".into()),
            //     ("r067000".into(),"r067000000".into()),
            //     ("r067500".into(),"r067500000".into()),
            //     ("r071400".into(),"r071400000".into()),
            //     ("r072500".into(),"r072500000".into()),
            //     ("r072800".into(),"r072800000".into()),
            //     ("r074100".into(),"r074100000".into()),
            //     ("r076500".into(),"r076500000".into()),
            //     ("r077900".into(),"r077900000".into()),
            //     ("r080500".into(),"r080500000".into()),
            //     ("r080700".into(),"r080700000".into()),
            //     ("r081500".into(),"r081500000".into()),
            //     ("r082600".into(),"r082600000".into()),
            //     ("r098200".into(),"r098200000".into()),
            // ]
        };
        let _ = utilites::serialize(s, super::CONF_FILE_NAME, false, utilites::Serializer::Toml);
    }
}
