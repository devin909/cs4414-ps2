use std::{libc, io, run, os, path, uint};
use std::io::WriterUtil;
use std::str;
fn leftpoint (p: ~str, mut args: ~[~str]){
    let command = args.remove(0);
    unsafe{
        let fd = do p.as_c_str |cstr| {
            libc::open(cstr, libc::O_RDONLY, 0)
        };
        run::Process::new(command, args, run::ProcessOptions{in_fd: Some(fd), out_fd: Some(1),.. run::ProcessOptions::new()});
    }

    
}
fn rightpoint(p: Path, mut args: ~[~str]){
    let command = args.remove(0);
    match io::file_writer(&p, [io::Create]) {
        Ok(writer)  => { writer.write_line(fmt!("%s", str::from_bytes(run::process_output(command, args).output))); }
        Err(err)    => {fail!(err)}
    }
}
fn pipe(mut args_left: ~[~str], mut args_right: ~[~str]){
    let mut command = args_left.remove(0);
    match io::file_writer(&Path("V4K-12@e.txt"), [io::Create]) {
        Ok(writer)  => { writer.write_line(fmt!("%s", str::from_bytes(run::process_output(command, args_left).output))); }
        Err(err)    => {fail!(err)}
    }
    command = args_right.remove(0);
    unsafe{
        let fd = do "V4K-12@e.txt".as_c_str |cstr| {
            libc::open(cstr, libc::O_RDONLY, 0)
        };
        run::Process::new(command, args_right, run::ProcessOptions{in_fd: Some(fd), out_fd: Some(1),.. run::ProcessOptions::new()});
    }
    os::remove_file(&Path("V4K-12@e.txt"));
}
fn main() {
    static CMD_PROMPT: &'static str = "gash > ";
    let mut hist: ~[~str] = ~[];
    let mut commCount = 0;
    loop {
        print(CMD_PROMPT);
        let line = io::stdin().read_line();
        debug!(fmt!("line: %?", line));
        let mut argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
                                 .transform(|x| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        hist.push((fmt!("%d ", commCount)) + line);
        let mut loop_exec = false;
        let mut lastind=0;
        for uint::range(0, argv.len()) |i| {
            //println(argv[i]);
            if (argv[i]==~"<"){
                //println ("<");
                let mut temp: ~[~str] = ~[];
                for uint::range(0,i)|j|{
                    temp.push(argv[j].clone());
                }
                let path_s = argv[i+1].clone();
                leftpoint(path_s, temp);
                loop_exec=true;
            }
            else if (argv[i]==~"|"){
                println ("|");
                let mut temp_left: ~[~str]=~[];
                let mut temp_right: ~[~str]=~[];
                for uint::range(0,i)|j|{
                    temp_left.push(argv[j].clone());
                }
                for uint::range(i+1,argv.len())|k|{
                    temp_right.push(argv[k].clone());
                }
                pipe(temp_left,temp_right);
                loop_exec=true;
            }
            else if (argv[i]==~">"){
                let mut temp: ~[~str] = ~[];
                for uint::range(0,i)|j|{
                    temp.push(argv[j].clone());
                }
                rightpoint(Path(argv[i+1]), temp);
                lastind=i+1;
                loop_exec=true;
                
            }
        }
        if (argv.len() > 0 && loop_exec==false) {
            let program = argv.remove(0);
            match program {
                ~"exit"     => {return; }
                ~"cd"       => {
                                    if (argv.len()==0){
                                        os::homedir();
                                    }
                                    else {
                                        os::change_dir(~path::Path(argv.remove(0)));
                                    }
                                }
                ~"history" => {
                                    for uint::range(0, hist.len()) |k| {
                                        println (hist[k]);
                                    }
                                }
                ~"histclear" =>{
                                    for uint::range(0,hist.len()) |hc|{
                                        hist.remove(0);
                                    }
                                    commCount=-1;

                }
                _           => {
                                    if (argv.len()>=1 && argv[argv.len()-1]==~"&"){
                                        let ampindex = argv.len()-1;
                                        argv.remove(ampindex);
                                        let arg: ~[~str] = argv;
                                        do spawn {

                                            run::process_status(program,arg);
                                        }
                                    }
                                    else{
                                        //println ("run fg");
                                        run::process_status(program, argv);
                                    }
                                }
            }
        }
        commCount+=1;
    }
}
