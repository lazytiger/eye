use console::Term;

fn main() ->anyhow::Result<()>{
    let term = Term::buffered_stdout();
    term.write_line("Hello, world!")?;
    term.flush()?;
    let line = term.read_line_initial_text(">")?;
    println!("{}", line);
    Ok(())
}