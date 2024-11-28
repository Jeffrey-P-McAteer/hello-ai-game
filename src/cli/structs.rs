
// See docs for clap's derive implementations at
//   https://docs.rs/clap/latest/clap/_derive/index.html#overview
#[derive(Debug, Clone, clap::Parser, Default, bevy::ecs::system::Resource)]
pub struct Args {
    /// Amount of verbosity in printed status messages; can be specified multiple times (ie "-v", "-vv", "-vvv" for greater verbosity)
    #[arg(short = 'v', long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// If set, every random-number generator will use this as their seed to allow completely deterministic AI runs.
    #[arg(short, long)]
    pub random_seed: Option<usize>,

    /// If this flag is passed the program outputs connected compute hardware and exits.
    #[arg(long, action=clap::ArgAction::SetTrue)]
    pub list_connected_hardware: bool,

    /// Pass a string to prompt the game's LLM agent w/ a string, compute result, and exit.
    #[arg(long)]
    pub test_llm_prompt: Option<String>,

    /// Pass a string to prompt the game's Image AI agent w/ a string, compute result, and exit. Image will always be saved to "out.png" in the CWD.
    #[arg(long)]
    pub test_image_prompt: Option<String>,

    /// Pass a file path to a custom *.onnx file and load that for LLM prompting
    #[arg(long)]
    pub llm_onnx_file: Option<String>,
    /// Pass a path to a tokenizer.json file to use that when translating prompt text and responses to/from text
    #[arg(long)]
    pub llm_tokenizer_json_file: Option<String>,


}
