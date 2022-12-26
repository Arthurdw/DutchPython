#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::Command;
#[cfg(not(unix))]
use std::process::exit;

use which::which;

use util::file_util;

use crate::util::json;

mod util;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

const TRANSLATIONS: &str = include_str!("../data/pairs.json");

fn get_translations() -> Vec<(String, String)> {
    let json = json::parse_json(TRANSLATIONS);
    let mut translations = json::into_collection(json::into_hashmap(json));
    translations.sort_by(|a, b| a.1.len().cmp(&b.1.len()).reverse());

    translations
}

fn get_passed_arguments() -> Vec<String> {
    std::env::args().collect()
}

fn main() {
    let translations = get_translations();
    let args = get_passed_arguments();

    if args.len() < 2 {
        println!("DutchPython - {} - {}\n", VERSION, AUTHOR);
        println!("Je moet de bestandsnaam van je applicatie meegeven.");
        println!("(vertaal <bestandsnaam>)");
        println!("Voorbeeld: vertaal applicatie.dpy");
        return;
    }

    let filename = &args[1];
    let mut content = file_util::read_file(&filename);
    for (english, dutch) in translations {
        content = content.replace(&dutch, &english);
    }

    let filename_without_ext = match filename.rsplit_once('.') {
        Some(spitted) => spitted.0,
        None => filename,
    };

    let out_filename = format!("{}.py", filename_without_ext);

    file_util::write_file(&out_filename, content.as_str());

    let python_path = which("python").expect("Kon geen Python interpreter vinden, vraag hulp aan een coach!");

    #[cfg(unix)]
    Command::new(python_path).arg(&out_filename).exec();

    #[cfg(not(unix))]
    {
        // Since we cannot replace our current process with the python process,
        // we will bubble up the exit code of the python process and run it as a child process.
        let mut child = Command::new(python_path).arg(&out_filename).spawn().unwrap();
        let exit_status = child.wait().unwrap();
        exit(exit_status.code().unwrap());
    }
}
