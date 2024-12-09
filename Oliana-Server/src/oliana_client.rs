
#![allow(unused_imports, unused_variables)]

use clap::Parser;
use clap::CommandFactory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt  = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(std::cmp::max(2, num_cpus::get_physical())) // Use all host cores, unless single-cored in which case pretend to have 2
    .thread_stack_size(8 * 1024 * 1024)
    .enable_time()
    .enable_io()
    .build()?;

  rt.block_on(async {
    if let Err(e) = main_async().await {
      eprintln!("[ main_async ] {}", e);
      std::process::exit(1);
    }
  });

  Ok(())
}

async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse().assign_some_defaults();
  if args.command == Command::Help {
    let mut help_cmd = Args::command();
    help_cmd.print_long_help()?;
    return Ok(());
  }

  println!("Connecting to {:?}", args.server_url);

  let mut transport = tarpc::serde_transport::tcp::connect(args.server_url, tarpc::tokio_serde::formats::Bincode::default);
  transport.config_mut().max_frame_length(usize::MAX);

  // OlianaClient is generated by the service attribute. It has a constructor `new` that takes a
  // config and any Transport as input.
  let client = oliana_server_lib::OlianaClient::new(tarpc::client::Config::default(), transport.await?).spawn();

  if args.command == Command::Text {
    let text_begin_diagnostic = client.generate_text_begin(tarpc::context::current(), args.prompt.clone() ).await?;
    eprintln!("From Server: {:?}", &text_begin_diagnostic);
    let mut generated_text = String::with_capacity(4096);
    while let Some(next_token) = client.generate_text_next_token(tarpc::context::current()).await? {
      generated_text.push_str(&next_token);
      generated_text.push_str(" ");
    }
    if args.output.len() > 0 {
      eprintln!("Writing {} chars to {}", generated_text.len(), &args.output);
      tokio::fs::write(&args.output, &generated_text).await?;
    }
  }
  else if args.command == Command::Image {
    //let reply = client.hello(tarpc::context::current(), format!("1")).await?;

  }

  Ok(())
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
pub enum Command {
  Text, Image,
  Help
}

impl Default for Command {
  fn default() -> Self { Command::Help }
}

// See docs for clap's derive implementations at
//   https://docs.rs/clap/latest/clap/_derive/index.html#overview
#[derive(Debug, Clone, clap::Parser, Default)]
pub struct Args {

   #[clap(value_enum, default_value_t=Command::Help)]
    pub command: Command,

    /// Pass a prompt for the Command's Image or Text AI agent (required for Image and Text commands)
    #[arg(short, long, default_value="")]
    pub prompt: String,

    /// File path to write Image or Text AI response back to (defaults to out.png when using Image command, writes to stdout if unspecified in Text command)
    #[arg(short, long, default_value="")]
    pub output: String,

    /// Hostname and port to connect to
    #[arg(short, long, default_value="localhost:9050")]
    pub server_url: String,

    /// Amount of verbosity in printed status messages; can be specified multiple times (ie "-v", "-vv", "-vvv" for greater verbosity)
    #[arg(short = 'v', long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// If set, every random-number generator will use this as their seed to allow completely deterministic AI runs.
    #[arg(short, long)]
    pub random_seed: Option<usize>,

}

impl Args {
  pub fn assign_some_defaults(mut self) -> Self {
    if self.command == Command::Image && self.output.len() < 1 {
      eprintln!("No --output specified in Image mode, defaulting to 'out.png'");
      self.output = "out.png".into();
    }
    self
  }
}

