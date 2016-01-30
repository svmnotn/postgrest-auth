#![allow(dead_code, unused_variables, unused_imports)]
#[macro_use]
extern crate clap;
use clap::{App, AppSettings};

extern crate iron;
use iron::prelude::*;
use iron::status;

extern crate serde_json;
extern crate jwt;

fn main() {
  // Create what is used to interpretate user input from the CLI
  let matches = App::new("postgrest-auth")
    .version(crate_version!())
    .author("Victor M. Suarez <svmnotn@gmail.com>, Caleb Meredith <calebmeredith8@gmail.com>")
    .about("An authentication server for PostgREST")
    .settings(&[AppSettings::GlobalVersion, AppSettings::UnifiedHelpMessage])
    .args_from_usage(
     "<URL>                           'a PostgreSQL database connection string'
      --port=[PORT]                -p 'The port on which the server will listen for HTTP requests. Defaults to 3001.'
      --user-relation=[USER]       -u 'The relation used to authenticate users. Defaults to postgrest.users'
      --refresh-relation=[REFRESH] -r 'The relation where refresh tokens will be stored. Defaults to postgrest.refresh'
      --grant-issuer=[ISSUER]      -i 'The role(s) to be given insert rights on the refresh relation.'
      --pass-regex=[PASS]          -w 'A regular expression for validating pass properties on users. Defaults to .{6,}'
      --jwt-expire=[EXPIRE]        -e 'The relative time it takes for a JWT to expire. Defaults to 30 minutes.'
      --jwt-secret=[SECRET]        -j 'The secret to use when encrypting JWTs. Defaults to secret.'
      --camelcase                  -c 'Camelcases everything in the REST interface.'")
    .get_matches();

  // Get CLI Options
  let url = value_t_or_exit!(matches.value_of("URL"), String);
  let port = value_t!(matches.value_of("port"), String).unwrap_or(String::from("3001"));
  let user = value_t!(matches.value_of("user-relation"), String).unwrap_or("postgrest.users".to_string());
  let refresh = value_t!(matches.value_of("refresh-relation"), String).unwrap_or("postgrest.refresh".to_string());
  let issuer = value_t!(matches.value_of("grant-issuer"), String);
  let regex = value_t!(matches.value_of("pass-regex"), String).unwrap_or(".{6,}".to_string());
  let expire = value_t!(matches.value_of("jwt-expire"), String).unwrap_or("30m".to_string());
  let secret = value_t!(matches.value_of("jwt-secret"), String).unwrap_or("secret".to_string());
  let camelcase = matches.is_present("camelcase");
  // Start the server
  let connection: &str = &(String::from("localhost:") + &port);
  let server = Iron::new(|req: &mut Request| {
    println!("REQUEST!: {}", req);
    Ok(Response::with((status::Ok, "Hello World!")))
  }).http(connection)
    .unwrap();
}
