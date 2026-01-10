use lightspeedvalidator::{Cli, CliExecutor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match &cli.command {
        lightspeedvalidator::Commands::Analyze(args) => {
            CliExecutor::execute(args.clone())?;
        }
    }
    
    Ok(())
}