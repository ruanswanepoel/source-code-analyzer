use std::io::BufRead;

struct ProjectOverview {
    files: u32,
    folders: u32,
    lines: u32,
    code_lines: u32,
    comments: u32,
    blanks: u32,
}

fn main() {
    let matches = clap::App::new("sce")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Source code evaluator")
        .arg(
            clap::Arg::new("path")
                .help("Path to the source code")
                .required(true)
                .index(1),
        )
        .get_matches();

    if let Some(path) = matches.value_of("path") {
        let overview = evaluate(path);
        match overview {
            Ok(o) => print_result(o),
            Err(err) => println!("{}", err),
        }
    }
}

fn evaluate(path: &str) -> Result<ProjectOverview, &'static str> {
    let mut overview = ProjectOverview {
        files: 0,
        folders: 0,
        lines: 0,
        code_lines: 0,
        comments: 0,
        blanks: 0,
    };

    if !std::path::Path::new(path).exists() {
        return Err("Could not find the path specified");
    }

    let mut dir_stack: Vec<String> = vec![String::from(path)];

    while !dir_stack.is_empty() {
        let current = dir_stack.pop().unwrap();
        let dir = std::fs::read_dir(current).unwrap();
        for p in dir {
            let p = p.unwrap();
            let path = p.path();
            if path.is_dir() {
                let pstr = p.path();
                match pstr.to_str() {
                    Some(val) => {
                        overview.folders += 1;
                        dir_stack.push(String::from(val))
                    }
                    None => {}
                }
            } else {
                let file = std::fs::File::open(path).unwrap();
                let reader = std::io::BufReader::new(file);

                overview.files += 1;

                for line in reader.lines() {
                    overview.lines += 1;
                    let s = line.unwrap().to_string();

                    if s.trim().is_empty() {
                        overview.blanks += 1;
                    } else if s.trim().starts_with("/") || s.trim().starts_with("*") {
                        overview.comments += 1;
                    } else {
                        overview.code_lines += 1;
                    }
                }
            }
        }
    }

    return Ok(overview);
}

fn print_result(overview: ProjectOverview) {
    println!("Files: {}", overview.files);
    println!("Folders: {}", overview.folders);
    println!("Lines: {}", overview.lines);
    println!("Code lines: {}", overview.code_lines);
    println!("Comments: {}", overview.comments);
    println!("Blanks: {}", overview.blanks);
}
