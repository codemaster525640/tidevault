use tv_engine::Engine;
use tv_kernel::clock::SystemClock;
use tv_kernel::money::{Currency, Money};
use rust_decimal::Decimal;

struct Cli {
    cmd: Commands,
}

enum Commands {
    DemoSeed { n: usize },
    FeeQuote { amount: Decimal, currency: String },
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let mut engine = Engine::new(SystemClock);
    match cli.cmd {
        Commands::DemoSeed { n } => {
            let seeded = engine.demo_seed(n)?;
            println!("seeded={seeded}");
        }
        Commands::FeeQuote { amount, currency } => {
            let ccy: Currency = currency.parse()?;
            let money = Money::new(amount, ccy)?;
            let fee = engine.pricing.fee_quote(money)?;
            println!("{fee}");
        }
    }
    Ok(())
}
