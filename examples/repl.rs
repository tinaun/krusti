extern crate krusti;
extern crate rustyline;


fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        let mut k_sess = krusti::Interpreter::new();

        match readline {
            Ok(ref line) if line != "\\q" => {
                rl.add_history_entry(&line);

                if line.len() > 0 {
                    let _ = k_sess.evaluate(&line);

                }
            },
            _ => break,
        }
    }
}
