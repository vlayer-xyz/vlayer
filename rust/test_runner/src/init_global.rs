use eyre::Result;
use foundry_cli::{handler, opts::GlobalOpts, utils};

pub fn init_global(global: &GlobalOpts) -> Result<()> {
    handler::install();
    utils::load_dotenv();
    utils::enable_paint();
    global.init()
}
