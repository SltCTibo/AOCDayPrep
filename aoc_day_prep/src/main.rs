use std::{env, fs::{File, OpenOptions}, io::{self, Write}, process::Command};

use regex::Regex;
use reqwest::header;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return
    }

    args.remove(0);

    for arg in &args {
        // Initier le dossier pour coder
        let file_name = format!("day{}", arg);
        Command::new("cargo").arg("new").arg(format!("./{}", &file_name)).spawn().expect("Should have created the cargo repo");

        // Get l'input + mettre dans input.txt
        let url = format!("https://adventofcode.com/2024/day/{}", arg);
        let mut request_headers = header::HeaderMap::new();
        request_headers.insert(header::COOKIE, header::HeaderValue::from_static("session=53616c7465645f5f7be6b31420f95d3fc8640f810bcea988a910ca78ec86079edd5beb328f444f8d152edae3a57d8ec98544370a0543d179b073cabf8cc3e126"));
        let client = reqwest::blocking::ClientBuilder::new()
            .default_headers(request_headers)
            .cookie_store(true)
            .build()
            .unwrap();
        let res = client.get(format!("{}/input", &url)).send().expect("Expect to get the Input");
        let body = res.text().expect("body invalid");
        let mut out = File::create(format!("./{}/input.txt", &file_name)).expect("Should have created the input file");
        io::copy(&mut body.as_bytes(), &mut out).expect("Should have copied into input");

        let res = client.get(url).send().expect("Expect to get the Input");
        let body = res.text().expect("body invalid");

        let re = Regex::new(r"<h2>--- (.*?) ---</h2>").unwrap();
        if let Some(captures) = re.captures(body.as_str()) {
            if let Some(title) = captures.get(1) {
                let formatted_content = format!("# {}\n\n## Part 1:\n\n## Part 2:", title.as_str());
                // Créer le Instrcutions.md
                let mut instr = File::create(format!("./{}/Instructions.md", &file_name)).expect("Should have created the instruction file");
                io::copy(&mut formatted_content.as_bytes(), &mut instr).expect("Expect to write in the Instructions");

                // Remplir le README.md
                let formatted_content = format!("\n## {} [go](./{}/Instructions.md)\n", title.as_str(), &file_name);
                let mut readme = OpenOptions::new()
                    .append(true)
                    .open("./README.md")
                    .expect("Should have opened README");
                readme.write_all(&mut formatted_content.as_bytes()).expect("Should have wrote in the readme");
            }
        }

        // Ecrire la ligne de contents dans le src/main.rs
        let mut main = File::create(format!("./{}/src/main.rs", &file_name)).expect("Should have filled the main file");
        let code = "fn main() {\n\tlet contents = std::fs::read_to_string(\"./input.txt\").expect(\"Should have opened the input file\");\n\n\tfor line in contents.lines() {\n\t\t// Insert code here\n\t}\n}";
        io::copy(&mut code.as_bytes(), &mut main).expect("Expect to write in the Instructions");

        Command::new("zed").arg(format!("./{}", &file_name)).spawn().expect("Should have created the cargo repo");
    }
}
