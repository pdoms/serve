use std::path::PathBuf;
use std::env;
use std::process::exit;

const DEF_HOST: &str = "localhost"; 
const DEF_PORT: &str = "8000"; 
const DEF_ASSETS: &str = "dist"; 

#[derive(Debug)]
pub struct Cli {
    pub host: String,
    pub port: String,
    pub address: String,
    pub index: PathBuf, 
}

impl std::fmt::Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(r#"
Cli Settings:
    host: {h}
    port: {p}
    address: {a}
    index: {i}
"#, h = self.host, p = self.port, a = self.address, i = self.index.to_str().unwrap()))
    }
}

fn useage(err: Option<&str>) {
    println!(r#"Usage: 
    ./server_app [FLAGS...]
        -u/--help.................. prints this usage;
        -h <host>.................. defines the host [DEFAULT: localhost];         
        -p <port>.................. defines the port [DEFAULT: 8000];   
        -s <src>................... defines the source directory [DEFAULT: "dist"];
             "#);
    if let Some(e) = err {
        println!("ERROR: {e}");
        exit(1)
    }
    exit(0)
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            host: DEF_HOST.to_string(),
            port: DEF_PORT.to_string(),
            address: format!("{DEF_HOST}:{DEF_PORT}"),
            index: PathBuf::from(DEF_ASSETS), 
        }
    }
}

impl Cli {
    pub fn from_args() -> Self {
        let mut cli = Cli::default();
        let mut args = env::args();
        args.next();
        loop {
            if let Some(arg) = args.next() {
                match arg.as_str() {
                    "-u" | "--help" => useage(None),
                    "-h" => {
                        if let Some(host) = args.next() {
                            cli.host = host;
                        } else {
                            useage(Some(&format!("missing argument 'host'")))
                        }
                    },
                    "-p" => {
                        if let Some(port) = args.next() {
                            cli.port = port;
                        } else {
                            useage(Some(&format!("missing argument 'port'")))
                        }
                    },
                    "-s" => {
                        if let Some(index) = args.next() {
                            cli.index = PathBuf::from(index);
                        } else {
                            useage(Some(&format!("missing argument 'source'")))
                        }
                    }
                    _ => useage(Some(&format!("unknown argument {arg}")))

                }
            } else {
                cli.address = format!("{h}:{p}", h = cli.host, p = cli.port);
                println!("Will run server at: {}", cli.address);
                break cli
            }
        }
    }
}


