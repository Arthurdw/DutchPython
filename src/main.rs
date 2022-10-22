use std::{
    fs::File,
    io::{Read, Write},
    os::unix::process::CommandExt,
    process::Command,
};

fn get_translations<'a>() -> Vec<(&'a str, &'a str)> {
    let mut translations = vec![
        ("def", "functie"),
        ("if", "indien"),
        ("else", "anders"),
        ("elif", "anders indien"),
        ("False", "onwaar"),
        ("True", "waar"),
        ("None", "niks"),
        ("and", "en"),
        ("as", "als"),
        ("assert", "vergelijk"),
        ("break", "onderbreek"),
        ("class", "klas"),
        ("continue", "ga door"),
        ("del", "verwijder"),
        ("except", "uitzondering"),
        ("finally", "uiteindelijk"),
        ("for", "voor elke"),
        ("from", "uit"),
        ("global", "globaal"),
        ("import", "importeer"),
        ("in", "in"),
        ("is", "is"),
        ("lambda", "anonieme functie"),
        ("nonlocal", "niet lokaal"),
        ("not", "niet"),
        ("or", "of"),
        ("pass", "laat door"),
        ("raise", "uitzonder"),
        ("return", "retour"),
        ("try", "probeer"),
        ("while", "terwijl"),
        ("with", "met"),
        ("yield", "beng op"),
        ("input", "vraag"),
        ("print", "toon"),
        ("range", "bereik"),
    ];
    translations.sort_by(|a, b| a.1.len().cmp(&b.1.len()).reverse());

    translations
}

fn read_file(path: &str) -> String {
    let mut file =
        File::open(path).expect("Kon het gevraagde bestand niet lezen, vraag hulp aan een coach!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Kon de text in het bestand niet als text lezen, vraag hulp aan een coach!");
    contents
}

fn write_file(path: &str, contents: &str) {
    let mut file =
        File::create(path).expect("Kon geen nieuw bestand aanmaken, vraag hulp aan een coach!");
    file.write_all(contents.as_bytes())
        .expect("Kon geen text in het bestand schrijven, vraag hulp aan een coach!")
}

fn get_passed_arguments() -> Vec<String> {
    std::env::args().collect()
}

fn main() {
    let translations = get_translations();
    let args = get_passed_arguments();

    if args.len() < 2 {
        println!("Je moet de bestandsnaam van je applicatie meegeven.");
        println!("(vertaal <bestandsnaam>)");
        println!("Voorbeeld: vertaal applicatie.dpy");
        return;
    }

    let filename = &args[1];
    let mut content = read_file(&filename);
    for (english, dutch) in translations {
        content = content.replace(&dutch, &english);
    }

    let filename_without_ext = match filename.rsplit_once('.') {
        Some(spitted) => spitted.0,
        None => filename,
    };

    let out_filename = format!("{}.py", filename_without_ext);

    write_file(&out_filename, content.as_str());
    Command::new("python").arg(&out_filename).exec();
}
