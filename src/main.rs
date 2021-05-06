use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {   
	print_header();

	let args: Vec<String> = env::args().collect();
	let f;

	match args.len() {
        2 => {
            f = &args[1];
            let file = File::open(f).unwrap();
    		let reader = BufReader::new(file);

    		let client = reqwest::Client::builder().build()?;

			for (_, uri) in reader.lines().enumerate() {

		        let uri = uri.unwrap(); // Ignore errors for now.
		        

		        if uri.starts_with("https://") || uri.starts_with("http://") {
			        
				    let res = client.get(&uri).send().await?;

			        match res.status() {
				        reqwest::StatusCode::BAD_REQUEST => println!(
				            "content-length:{:?} server:{:?}", 
				            res.headers().get(reqwest::header::CONTENT_LENGTH),
				            res.headers().get(reqwest::header::SERVER),
				        ),
				        status => println!("{}:{}", uri, status),
				    }

		        } else {
		        	// when a schema is not specified we MUST test both http/s
		        	
		        	let http_uri = format!("{}{}", "http://", &uri);
		        	let https_uri = format!("{}{}","https://", &uri);


			    	let http_res = client.get(&http_uri).send().await?;

			        match http_res.status() {
				        reqwest::StatusCode::BAD_REQUEST => println!(
				            "content-length:{:?} server:{:?}", 
				            http_res.headers().get(reqwest::header::CONTENT_LENGTH),
				            http_res.headers().get(reqwest::header::SERVER),
				        ),
				        status => println!("{}:{}", http_uri, status),
				    }


			    	let https_res = client.get(&https_uri).send().await?;

			        match https_res.status() {
				        reqwest::StatusCode::BAD_REQUEST => println!(
				            "content-length:{:?} server:{:?}", 
				            https_res.headers().get(reqwest::header::CONTENT_LENGTH),
				            https_res.headers().get(reqwest::header::SERVER),
				        ),
				        status => println!("{}:{}", https_uri, status),
				    }
		        }
		        
    		}

		    Ok(())
        },
        _ => {
            print_help();
            process::exit(1);
        }
    }

}

fn print_header() {
    println!("");
    println!("");
	println!("\t############################################");
    println!("\tValidate your domains!");
    println!("\t############################################");

    println!("");
    println!("");
}

fn print_help() {
    println!("usage:");
	println!("rustydomain <f>");
}


#[test]
fn it_works(){
	assert_eq!(true, true);
}