use std::fs;
use std::path::PathBuf;
use std::env;

struct GerberExtention<'a> {
    from: &'a str,
    to: &'a str
}

const GERBER_EXTENTIONS: [GerberExtention; 9] = [
    GerberExtention{from: "-B.SilkS.gbr", to: ".GBO"},
    GerberExtention{from: "-B.Mask.gbr", to: ".GBS"},
    GerberExtention{from: "-B.Cu.gbr", to: ".GBL" },
    GerberExtention{from: "-F.Cu.gbr", to: ".GTL" },
    GerberExtention{from: "-F.Mask.gbr", to: ".GTS" },
    GerberExtention{from: "-F.SilkS.gbr", to: ".GTO" },
    GerberExtention{from: "-Edge.Cuts.gbr", to: ".GKO" },
    GerberExtention{from: "-NPTH.TXT", to: "-NPTH.TXT" },
    GerberExtention{from: "-PTH.TXT", to: "-PTH.TXT" }
];

struct FileData<'a> {
    file_name: String,
    path: PathBuf,
    ext: &'a GerberExtention<'a>
}

fn find_by_extention( file_data: &Vec<FileData>, ext: &GerberExtention) -> Option<usize> {
    file_data.iter().position(|x| x.ext.to == ext.to)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path;
    if args.len() > 1 {
        path = PathBuf::from(&args[1]);
    } else {
        // path = env::current_dir().unwrap();
        println!("Enter dir path!");
        return
    }

    if !path.is_dir() {
        println!("This is not dir or not found : {:?}", path);
        return
    }

    println!("Open : {:?}\n", path);

    let dir_entries = fs::read_dir(path).unwrap();
    let mut file_data: Vec<FileData> = Vec::new();

    for entry in dir_entries {
        let file = entry.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.into_string().unwrap();
        for ext in GERBER_EXTENTIONS.iter() {
            if let Some(_) = file_name.find(ext.from) {
                if let Some(pos) = find_by_extention(&file_data, &ext) {
                    println!("{} is already found! {}, {}", ext.from, file_data[pos].file_name, file_name);
                    return
                }
                file_data.push(FileData{file_name: file_name.clone(), path: file.path(), ext: ext});
            }
        }
    }

    let mut has_missing_file = false;

    for ext in GERBER_EXTENTIONS.iter() {
        if let Some(_) = find_by_extention(&file_data, &ext) {
            // println!("{} --- {}", ext.to, file_data[i].file_name);
        } else {
            println!("!!!!! {} is not found !!!!!!  ", ext.from);
            has_missing_file = true;
        }
    }

    if has_missing_file {
        println!("Some files are not found!");
        return
    }

    for data in file_data {
        let new_file_name = data.file_name.replace(data.ext.from, data.ext.to);
        let mut new_path = PathBuf::from(&data.path);
        new_path.set_file_name(&new_file_name);
        // println!("{:?}", new_path);
        match fs::rename(data.path, new_path) {
            Ok(_) => println!("{} -> {}", data.file_name, new_file_name),
            Err(err) => println!("Error: {}", err),
        };
    }

    println!("Done!");
}
