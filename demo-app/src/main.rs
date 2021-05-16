pub mod app;
mod di;

fn main() -> Result<(), String> {
    let res = di::init();
    if let Err(e) = res {
        return Err(e);
    }

    Ok(())
}
