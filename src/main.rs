use std::{fs, path::PathBuf};


fn main()
{
    rename();
}

fn rename() 
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2
    {
        println!("⚠️ Ошибка, не передано имя файла с картой переименования");
        std::process::exit(0x0100);
    }
    let file_name = &args[1];
    let mut source_name : Vec<PathBuf> = vec![];
    let mut target_name : Vec<PathBuf> = vec![];
    let fl = fs::read_to_string(file_name);
    let current_dir = std::env::current_dir().expect("⚠️ Невомзможно определить текущую директорию!");
    if let Ok(file) = fl
    {
       for l in file.lines()
       {
        if !l.contains(',')
        {
            println!("⚠️ Ошибка, значение {} не содержит раделителя - , (зпт)", l);
            std::process::exit(0x0100);
        }
            let sp : Vec<&str> = l.split(",").collect();
            let mut dir_1 = current_dir.clone();
            let mut dir_2 = current_dir.clone();
            dir_1.push(sp[0].trim());
            dir_2.push(sp[1].trim());

            if !dir_1.exists()
            {
                if dir_2.exists()
                {
                    println!("😳 Директория {} уже имеет конечное наименование из файла сопоставления", dir_2.display());
                }
                else
                {
                    println!("⚠️ Ошибка, директория {} не существует!", dir_1.display());
                    std::process::exit(0x0100);
                }
            }
            else
            {
                source_name.push(dir_1);
                target_name.push(dir_2);
            }

            for (i, dir) in source_name.iter().enumerate()
            {
                let target = target_name.iter().nth(i).expect(&format!(" ⚠️Директория соспоставляемая с {} не найдена", dir.display()));
                if cfg!(windows) 
                {
                    if let Err(e) = fs::rename(&dir, target)
                    {
                        fs::rename(&dir, target);
                    }
                } 
                else if cfg!(unix) 
                {
                    fs::rename(&dir, target).expect(format!("⚠️ Ошибка переименования {} в {}", dir.display(), target_name[i].display()).as_str());
                }
                
                println!("😉 Переименование {} -> {}", dir.display(), target.display());
            }
       }
    }
    else
    {
        println!("⚠️ В директории {} файл переименования {} не найден. ({})", current_dir.display(), file_name, fl.err().unwrap());
    }
}