use std::{io, run, os, path};

fn main() {
    static CMD_PROMPT: &'static str = "gash > ";
    
    loop {
        print(CMD_PROMPT);
        let line = io::stdin().read_line();
        debug!(fmt!("line: %?", line));
        let mut argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
                                 .transform(|x| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        
        if argv.len() > 0 {
            let program = argv.remove(0);
            match program {
                ~"exit"     => {return; }
                //~"cd"       => {os::change_dir(Path.from_str(argv.remove(1)));}
                ~"cd"       => {
                                    //let paths: path = argv.remove(1); 
                                    os::change_dir(~path::Path(argv.remove(0)));
                                    //let a =path::dirname(argv.remove(1));
                                }
                _           => {run::process_status(program, argv);}
            }
        }
    }
}
