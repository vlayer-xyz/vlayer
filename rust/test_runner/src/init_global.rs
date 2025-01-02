use eyre::Result;
use foundry_cli::{handler, utils};
use foundry_cli::opts::GlobalOpts;

pub fn init_global(global: &GlobalOpts) -> Result<()> {
    handler::install();
    utils::load_dotenv();
    utils::enable_paint();
    global.init()
}