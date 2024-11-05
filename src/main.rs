use arboard::Clipboard;
use clap::Parser;
use eyre::Context;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(name = "rid")]
#[command(about = "Utility for getting uuids")]
#[command(version = "0.1")]
struct ProgramArgs {
    /// Prints the ID instead of setting it in the clipboard.
    #[arg(short, long)]
    print: bool,
}

fn main() -> eyre::Result<()> {
    let args = ProgramArgs::parse();

    let id = Uuid::new_v4().to_string();

    if args.print {
        println!("{id}");
    } else {
        set_clipboard(id).wrap_err("Failed setting clipboard")?;
    }

    Ok(())
}

fn set_clipboard(text: String) -> eyre::Result<()> {
    let mut clipboard =
        Clipboard::new().map_err(|e| eyre::eyre!("Failed to create clipboard, err: {e:?}"))?;

    clipboard
        .set_text(text)
        .map_err(|e| eyre::eyre!("Failed to set clipboard context, err: {e:?}"))?;

    Ok(())
}
