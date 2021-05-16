use pulldown_cmark::{html, Options, Parser};
use std::{
    env,
    fs::{read_dir, File},
    io::{BufRead, BufReader, Error, Write},
};

fn main() -> Result<(), Error> {
    let current_dir = env::current_dir()?;
    let posts = read_dir(current_dir.join("posts"))?;
    let mut output = vec![];

    for file in posts {
        let mut render = String::new();
        let mut details = String::new();
        let post = File::open(file?.path())?;
        let buf = BufReader::new(post);
        let (mut seeking_details, mut parsing_details) = (true, false);

        for line in buf.lines() {
            let line = line?;

            if line == "---".to_string() && seeking_details {
                seeking_details = false;
                parsing_details = true;
                continue;
            } else if parsing_details {
                if line == "---".to_string() {
                    parsing_details = false;
                } else {
                    let (desc, body) = line.split_at(line.find(':').unwrap_or(0));
                    details.push_str(&format!(
                        "\t\t\t\"{}\": \"{}\",\n",
                        desc,
                        body.replace(":", "").trim().replace("'", "")
                    ));
                }
                continue;
            } else if seeking_details {
                seeking_details = false;
            }

            let (mut html_output, parser) =
                (String::new(), Parser::new_ext(&line, Options::empty()));
            html::push_html(&mut html_output, parser);
            render.push_str(&html_output.replace("\"", "\\\""));
        }

        output.push((details, render));
    }

    let mut json_file = File::create(current_dir.join("_posts.json"))?;
    writeln!(json_file, "{{\n\t\"posts\": [")?;
    let (mut no, last) = (0, output.len()-1);

    for (details, render) in output {
        let json = format!(
            "{}\t\t\t\"body\": \"{}\"",
            details,
            render.replace("\n", "\\n")
        );
        write!(json_file, "\t\t{{\n{}\n\t\t}}", json)?;
        if no != last {
            writeln!(json_file, ",")?;
            no += 1;
        } else {
            writeln!(json_file, "")?;
        }
    }

    writeln!(json_file, "\t]\n}}")?;

    Ok(())
}
