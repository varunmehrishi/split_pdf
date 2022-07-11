use clap::{AppSettings, Parser};
use std::{fmt::Display, process::Command, thread};

#[derive(Parser, Debug)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Args {
    #[clap(value_parser)]
    input_file: String,

    #[clap(short = 's', long, value_parser)]
    page_start: Option<u32>,

    #[clap(short = 'e', long, value_parser)]
    page_end: Option<u32>,
}

fn main() {
    let args = Args::parse();
    if let Some(file_name) = args.input_file.strip_suffix(".pdf") {
        let mut file_name = file_name.to_owned();

        if args.page_start.is_some() || args.page_end.is_some() {
            let mut command = get_trim_command(&file_name, args.page_start, args.page_end);

            if command.output().is_ok() {
                file_name = get_intermediate_filename(&file_name);
            }
        }

        let ofile_name = file_name.to_owned();
        let ot = thread::spawn(move || {
            let mut command = get_command(&ofile_name, PageList::Odd);
            command.output()
        });

        let even_result = get_command(&file_name, PageList::Even).output();

        let odd_result = ot.join();

        if odd_result.is_ok() && even_result.is_ok() {
            println!("Success!")
        } else {
            println!("Failed!")
        }

        if file_name.ends_with("_inter") {
            Command::new("rm")
                .arg(format!("{file_name}.pdf"))
                .output()
                .expect("Failed while removing intermediate file.");
        }
    } else {
        println!("File extension in not pdf.");
    }
}

enum PageList {
    Even,
    Odd,
}

impl Display for PageList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageList::Odd => write!(f, "odd"),
            PageList::Even => write!(f, "even"),
        }
    }
}

fn get_command(name: &str, page_list: PageList) -> Command {
    let mut command = Command::new("gs");

    command
        .arg("-sDEVICE=pdfwrite")
        .arg("-dBATCH")
        .arg("-dNOPAUSE")
        .arg("-dNEWPDF=false")
        .arg(format!("-sPageList={page_list}"))
        .arg(format!("-sOutputFile={name}_{page_list}.pdf"))
        .arg(format!("{name}.pdf"));

    command
}

fn get_trim_command(name: &str, page_start: Option<u32>, page_end: Option<u32>) -> Command {
    let mut command = Command::new("gs");

    command
        .arg("-sDEVICE=pdfwrite")
        .arg("-dBATCH")
        .arg("-dNOPAUSE")
        .arg("-dNEWPDF=false");

    if let Some(start) = page_start {
        command.arg(format!("-dFirstPage={start}"));
    }

    if let Some(end) = page_end {
        command.arg(format!("-dLastPage={end}"));
    }

    command
        .arg(format!(
            "-sOutputFile={}.pdf",
            get_intermediate_filename(name)
        ))
        .arg(format!("{name}.pdf"));

    command
}

fn get_intermediate_filename(file_name: &str) -> String {
    format!("{file_name}_inter")
}
