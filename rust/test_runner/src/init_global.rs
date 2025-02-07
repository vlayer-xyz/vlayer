use eyre::Result;
use foundry_cli::{handler, opts::GlobalArgs, utils};

pub fn init_global(global: &GlobalArgs) -> Result<()> {
    handler::install();
    utils::load_dotenv();
    utils::enable_paint();
    global.init()
}
