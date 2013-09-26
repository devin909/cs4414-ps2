use std::{io, run, os, path, uint};

fn main() {
    static CMD_PROMPT: &'static str = "gash > ";
    let mut hist: ~[~str] = ~[];
    let mut i = 0;
    loop {
        print(CMD_PROMPT);
        let line = io::stdin().read_line();
        debug!(fmt!("line: %?", line));
        let mut argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
                                 .transform(|x| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        hist.push((fmt!("%i ", i)) + line);
        
        if argv.len() > 0 {
            let program = argv.remove(0);
            match program {
                ~"exit"     => {return; }
                //~"cd"       => {os::change_dir(Path.from_str(argv.remove(1)));}
                ~"cd"       => {
                                    //let dir = argv.remove(0);
                                    if (argv.len()==0){
                                        os::homedir();
                                        //os::change_dir(~path::Path("/home/devin"));
                                    }
                                    else {
                                        os::change_dir(~path::Path(argv.remove(0)));
                                    }
                                    //let paths: path = argv.remove(1); 
                                    //os::change_dir(~path::Path(argv.remove(0)));
                                    //let a =path::dirname(argv.remove(1));
                                }
                ~"history" => {
                                    for uint::range(0, hist.len()) |k| {
                                        println (hist[k]);
                                    }
                }
                _           => {

                                    run::process_status(program, argv);
                                    //run::ProcessOutput();
                                }
            }
        }
        i+=1;
    }
}
