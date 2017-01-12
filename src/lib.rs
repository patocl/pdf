// #![feature(plugin)]
// #![plugin(clippy)]

#[macro_use (o, slog_log, slog_trace, slog_debug, slog_info, slog_warn, slog_error)]
extern crate slog;
extern crate slog_json;
#[macro_use]
extern crate slog_scope;
extern crate slog_stream;
extern crate slog_term;
extern crate isatty;
#[macro_use]
extern crate error_chain;

pub mod reader;
pub mod repr;
pub mod err;



// Plan:
// First don't care about storing structures. Just use Lexer to parse things whenever needed.


// Need goals!
// - Read Root/catalog
// - Read Pages


// Let's say a chain from Trailer - Catalog - Pages - Page
// can be make it optional whether we save each step and how far we follow the chain?
//  - PdfReader has higher-level functions. Implementation dictates whether it follows chain or
//  saves stuff!

#[cfg(test)]
mod tests {
    use reader::PdfReader;
    use repr::*;
use err::*;

    use std;
    use slog;
    use slog::{DrainExt, Level};
    use {slog_term, slog_stream, isatty, slog_json, slog_scope};

    const EXAMPLE_PATH: &'static str = "example.pdf";


    #[test]
    fn structured_read() {
        setup_logger();

        let reader = unwrap(PdfReader::new(EXAMPLE_PATH));

        {
            let val = reader.trailer.dict_get(String::from("Root"));

            if let Ok(&Object::Reference{obj_nr: 1, gen_nr: 0}) = val {
            } else {
                println!("Wrong Trailer::Root!");
                unwrap(val);
            }
        }

        {
        }

        unwrap(reader.read_indirect_object(3));
    }

    /// Prints the error if it is an Error
    fn unwrap<T>(err: Result<T>) -> T {
        match err {
            Ok(ok) => {ok},
            Err(err) => {
                println!("\n === \nError: {}", err);
                for e in err.iter().skip(1) {
                    println!("  caused by: {}", e);
                }
                println!(" === \n");
                panic!("Exiting");
            },
        }
    }


    fn setup_logger() {
        let logger = if isatty::stderr_isatty() {
            let drain = slog_term::streamer()
                .async()
                .stderr()
                .full()
                .use_utc_timestamp()
                .build();
            let d = slog::level_filter(Level::Trace, drain);
            slog::Logger::root(d.fuse(), o![])
        } else {
            slog::Logger::root(slog_stream::stream(std::io::stderr(), slog_json::default()).fuse(),
                               o![])
        };
        slog_scope::set_global_logger(logger);
    }
}
