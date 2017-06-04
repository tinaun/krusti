extern crate krusti;
extern crate rustyline;


fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(ref line) if line != "\\q" => {
                rl.add_history_entry(&line);

                if line.len() > 0 {
                    let i = krusti::krust::parse_item(&line);

                    match i {
                        Ok(item) => println!(" {}", item),
                        Err(e)   => {
                            // TODO: better spans
                            // println!("    {: >pad$}", '^', pad = e.span());
                            println!("Error: {}", e);
                        },
                    }
                }
            },
            _ => break,
        }
    }
}