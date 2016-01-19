#![allow(dead_code, unused_variables, unused_imports)]
#[macro_use]
extern crate clap;
use clap::App;

extern crate iron;
use iron::prelude::*;
use iron::status;

extern crate serde_json;
extern crate jwt;

fn main() {
  // Create what is used to interpretate user input from the CLI
  let matches = App::new("postgrest-auth")
    .version(&crate_version!())
    .global_version(true)
    .unified_help_message(true)
    .author("Victor M. Suarez <svmnotn@gmail.com>")
    .about("An authentication server for PostgREST")
    .args_from_usage(
     "<url>                             'a PostgreSQL database connection string'
      --port=[number]                -p 'The port on which the server will listen for HTTP requests. Defaults to 3001.'
      --user-relation=[uRelation]    -u 'The relation used to authenticate users. Defaults to postgrest.users'
      --refresh-relation=[rRelation] -r 'The relation where refresh tokens will be stored. Defaults to postgrest.refresh'
      --grant-issuer=[role]          -i 'The role(s) to be given insert rights on the refresh relation.'
      --pass-regex=[regex]           -w 'A regular expression for validating pass properties on users. Defaults to .{6,}'
      --jwt-expire=[time]            -e 'The relative time it takes for a JWT to expire. Defaults to 30 minutes.'
      --jwt-secret=[secret]          -j 'The secret to use when encrypting JWTs. Defaults to secret.'
      --camelcase                    -c 'Camelcases everything in the REST interface.'")
    .get_matches();

  // Get CLI Options
  let url = value_t_or_exit!(matches.value_of("url"), String);
  let port = value_t!(matches.value_of("number"), String).unwrap_or(String::from("3001"));
  let user = value_t!(matches.value_of("uRelation"), String).unwrap_or("postgrest.users".to_string());
  let refresh = value_t!(matches.value_of("rRelation"), String).unwrap_or("postgrest.refresh".to_string());
  let issuer = value_t!(matches.value_of("role"), String);
  let regex = value_t!(matches.value_of("regex"), String).unwrap_or(".{6,}".to_string());
  let expire = value_t!(matches.value_of("time"), String).unwrap_or("30m".to_string());
  let secret = value_t!(matches.value_of("secret"), String).unwrap_or("secret".to_string());
  let camelcase = matches.is_present("camelcase");
  // Start the server
  let connection: &str = &(String::from("localhost:") + &port);
  let server = Iron::new(|req: &mut Request| {
    println!("REQUEST!: {}", req);
    Ok(Response::with((status::Ok, "Hello World!")))
  }).http(connection)
    .unwrap();
}
