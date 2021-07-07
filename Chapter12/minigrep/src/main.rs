use std::env;
use std::fs;

static listing :u32 = 7; //choose a list to execute

fn main() {

    //Reading the Argument Values
    if listing == 1 {
        println!("Collecting the command line arguments into a vector and printing them");
        let arg : Vec<String> = env::args().collect();
        println!("{:?}",arg);
    } 

    //Saving the Argument Values in Variables
    else if listing == 2{
        println!("Creating variables to hold the query argument and filename argument");
        let arg : Vec<String> = env::args().collect();
        let query = &arg[1]; 
        let filename = &arg[2];

        println!("Searching for {}", query);
        println!("In {}", filename);
    }   
    //Reading a File
    else if listing == 4{
        println!("Reading the contents of the file specified by the second argument");
        let arg : Vec<String> = env::args().collect();
        let query = &arg[1];
        let filename = &arg[2];

        println!("In file {}",filename);

        let content = fs::read_to_string(filename).expect("Something went wrong when reading the file");

        println!("Content: \n{}",content);
    }
    //Separation of Concerns for Binary Projects
    else if listing == 5 {
        println!("Extracting a parse_config function from main");
        let arg : Vec<String> = env::args().collect();
        
        let (query,filename) = parse_config_5( &arg);
        println!("{}",query);
        println!("{}",filename);
    }

    else if listing ==6 {
        let args: Vec<String> = env::args().collect();
        let config = parse_config_6(&args);
        
        println!("Searching for {}", config.query);
        println!("In file {}", config.filename);
        let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    }
    else if listing == 7{
        println!("Changing parse_config into Config::new");
        let args: Vec<String> = env::args().collect();
        let config = Config_7::new(&args);
        println!("{}",config.query);
        println!("{}",config.filename);
    }

    //Fixing the Error Handling
    else if listing == 8{
        println!("Changing parse_config into Config::new");
        let args: Vec<String> = env::args().collect();
        let config = Config_7::new(&args);
        println!("{}",config.query);
        println!("{}",config.filename);
    }


}

//listing 5
fn parse_config_5(arg:&[String]) -> (&str,&str){
    let query = &arg[1];
    let filename = &arg[2];
    (query,filename)
}

//listing 6
struct Config_6{
    query : String,
    filename : String 
}


fn parse_config_6(arg:&[String])-> Config_6 {
    let query = arg[1].clone();
    let filename = arg[2].clone();
    Config_6{query,filename}
}

//listing 7
struct  Config_7{
    query :String,
    filename : String
}
impl Config_7{
    fn new(arg:&[String])-> Config_7{
        let query = arg[1].clone();
        let filename = arg[2].clone();
        Config_7{query,filename}
    }
}
//listing 8
