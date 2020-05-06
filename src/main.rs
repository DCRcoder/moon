use structopt::StructOpt;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;

use slog::Drain;

pub mod cfg;
pub mod moon;
pub mod error;
pub mod consts;

extern crate dirs;
#[macro_use]
extern crate log;


#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// add|a "THING I NEED TO DO +project @context"
    #[structopt(long)]
    add: Option<String>,

    /// addm "THINGS I NEED TO DO MORE THINGS I NEED TO DO"
    #[structopt(long)]
    addm: Option<String>,

    // /// addto DEST "TEXT TO ADD"
    // #[structopt(long)]
    // addto: String,

    // /// append|app ITEM# "TEXT TO APPEND"
    // #[structopt(long)]
    // append: String,

    // /// archive
    // #[structopt(long)]
    // archive: String,

    // /// command [ACTIONS]
    // #[structopt(long)]
    // command: String,

    // /// deduplicate
    // #[structopt(long)]
    // deduplicate: String,

    // /// del|rm ITEM# [TERM]
    // #[structopt(long)]
    // del: String,

    // /// depri|dp ITEM#[, ITEM#, ITEM#, ...]
    // #[structopt(long)]
    // depri: String,

    // /// done|do ITEM#[, ITEM#, ITEM#, ...]
    // #[structopt(long)]
    // done: String,

    // /// list|ls [TERM...]
    // #[structopt(long)]
    // list: String,

    // /// listaddons
    // #[structopt(long)]
    // listaddons: String,

    // /// listcon|lsc [TERM...]
    // #[structopt(long)]
    // listcon: String,

    // /// listfile|lf [SRC [TERM...]]
    // #[structopt(long)]
    // listfile: String,

    // /// listpri|lsp [PRIORITIES] [TERM...]
    // #[structopt(long)]
    // listpri: String,

    // /// listproj|lsprj [TERM...]
    // #[structopt(long)]
    // listproj: String,

    // /// move|mv ITEM# DEST [SRC]
    // #[structopt(long)]
    // mv: String,

    // /// prepend|prep ITEM# "TEXT TO PREPEND"
    // #[structopt(long)]
    // prepend: String,

    // /// pri|p ITEM# PRIORITY
    // #[structopt(long)]
    // pri: String,

    // /// replace ITEM# "UPDATED TODO"
    // #[structopt(long)]
    // replace: String,

    // /// report
    // #[structopt(long)]
    // report: String,
}

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let logger = slog::Logger::root(drain, o!());
    let _guard = slog_scope::set_global_logger(logger);

    slog_stdlog::init().unwrap();
    let current_home_dir = dirs::home_dir().unwrap();
    let cfg = cfg::Config::new(current_home_dir);
    info!("config:{:?}", cfg);
    let mut m = moon::Moon::new(cfg).unwrap();

    let opt = Opt::from_args();
    match opt.add {
        None => info!("add get nothing"),
        Some(add) => {
            let a = m.add(&add);
            match a {
                Ok(_) => info!("[add action successful]: todo:{}", &add),
                Err(e) => error!("error:{:?}", e)
            }
        }
    }
}
