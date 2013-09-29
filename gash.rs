use std::{io, run, os, path, uint};
use std::io::WriterUtil;
use std::str;
fn leftpoint (){
    
}
fn rightpoint(p: &Path, args: ~[~str]){
    let path = Path("aaa.txt");
    //println(fmt!("AAAA %s", str::from_bytes(run::process_output("ls", ~[]).output)));
    match io::file_writer(&path, [io::Create, io::Append]) {
        Ok(writer)  => { writer.write_line(fmt!("%s", str::from_bytes(run::process_output("ls", ~[]).output))); }
        Err(err)    => {fail!(err)}
    }
}
fn pipe(){
    
}
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
        let mut loop_exec = false;
        for uint::range(0, argv.len()) |i| {
            //println(argv[i]);
            if (argv[i]==~"<"){
                println ("<");
                loop_exec=true;
            }
            else if (argv[i]==~"|"){
                println ("|");
                loop_exec=true;
            }
            else if (argv[i]==~">"){
                //println (">");
                for uint::range(0,i) |j|{

                }
                //rightpoint();
                loop_exec=true;
            }
        }
        if (argv.len() > 0 && loop_exec==false) {
            let program = argv.remove(0);
            match program {
                ~"exit"     => {return; }
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
                                    if (argv.len()>=1 && argv[argv.len()-1]==~"&"){
                                        let ampindex = argv.len()-1;
                                        argv.remove(ampindex);
                                        let arg: ~[~str] = argv;
                                        //argv.remove(1);
                                        do spawn {

                                            run::process_status(program,arg);
                                        }
                                    }
                                    else{
                                        println ("run fg");
                                        run::process_status(program, argv);
                                        //println("AAA");
                                        //run::process_status("ls",~[]);
                                        //run::ProcessOutput();
                                    }
                                }
            }
        }
        i+=1;
    }
}
