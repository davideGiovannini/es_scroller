use indicatif::*;

use serde_json::Error;
use std::io;

// TODO try with reqwest and raw post request https://www.elastic.co/guide/en/elasticsearch/guide/1.x/scan-scroll.html
// Updated url https://www.elastic.co/guide/en/elasticsearch/guide/2.x/scroll.html

mod args;
mod elasticsearch;
pub use self::args::ScrollerOptions;
pub use self::elasticsearch::{errors::EsError, ScrollClient};

use std::fs::File;
use std::io::{stdout, Stdout, Write};
use std::path::Path;

enum FileOrStdout {
    File(File),
    Stdout(Stdout),
}

impl Write for FileOrStdout {
    fn write(&mut self, data: &[u8]) -> std::result::Result<usize, std::io::Error> {
        match self {
            FileOrStdout::File(ref mut file) => file.write(data),
            FileOrStdout::Stdout(ref mut stdout) => stdout.write(data),
        }
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        match self {
            FileOrStdout::File(ref mut file) => file.flush(),
            FileOrStdout::Stdout(ref mut stdout) => stdout.flush(),
        }
    }
}

impl FileOrStdout {
    fn is_stdout(&self) -> bool {
        matches!(self, FileOrStdout::Stdout(_))
    }
}

pub fn process(options: &ScrollerOptions) -> Result<(), EsError> {
    let print_function = if options.pretty {
        serde_json::to_string_pretty
    } else {
        serde_json::to_string
    };

    let stdout = stdout();
    //    let mut stdout_lock = stdout.lock();

    let output = if options.output != Path::new("-") {
        FileOrStdout::File(File::create(options.output.clone()).unwrap())
    } else {
        FileOrStdout::Stdout(stdout)
    };

    if let Some(limit) = options.limit {
        process_elements(
            ScrollClient::new(options)?.take(limit),
            output,
            &options.index,
            options.silent,
            print_function,
        )
    } else {
        process_elements(
            ScrollClient::new(options)?,
            output,
            &options.index,
            options.silent,
            print_function,
        )
    };
    Ok(())
}

fn process_elements<I>(
    scroll: I,
    mut output: FileOrStdout,
    index_name: &str,
    silent: bool,
    print_function: fn(&I::Item) -> Result<String, Error>,
) where
    I: std::iter::Iterator,
    I::Item: serde::Serialize,
{
    let progress_bar = ProgressBar::new(scroll.size_hint().0 as u64);

    let target = if output.is_stdout() || silent {
        ProgressDrawTarget::hidden()
    } else {
        ProgressDrawTarget::stderr()
    };

    progress_bar.set_draw_target(target);
    progress_bar.set_style(indicatif::ProgressStyle::default_bar()
        .template("{msg}\n[{elapsed_precise}] |{bar:40.cyan/blue}| {pos:>7}/{len:7} ETA: {eta_precise}").expect("Template error")
        .progress_chars("#>-"));

    progress_bar.set_message(format!("Downloading: {index_name}"));

    for item in progress_bar.wrap_iter(scroll) {
        let string = print_function(&item).unwrap();
        if let Err(er) = writeln!(&mut output, "{}", &string) {
            if er.kind() == io::ErrorKind::BrokenPipe {
                break;
            }
        }
    }
    progress_bar.finish_with_message(format!("Downloaded:  {index_name}"));
}
