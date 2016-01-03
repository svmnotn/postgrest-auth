#![allow(dead_code, unused_variables, unused_imports)]
#[macro_use]
extern crate clap;
use clap::App;

extern crate iron;
use iron::prelude::*;
use iron::status;

fn main() {
  // Create what is used to interpretate user input from the CLI
  let matches = App::new("postgrest-auth")
                  .version(&crate_version!())
                  .global_version(true)
                  .unified_help_message(true)
                  .author("Victor M. Suarez <svmnotn@gmail.com>")
                  .about("An authentication server for PostgREST")
                  .args_from_usage("<url>                             'a PostgreSQL database connection string'
                                    --port=[number]                -p 'The port on which the server will listen for HTTP requests. Defaults to 3001'
                                    --user-relation=[uRelation]    -u 'The relation (table or view) which PostgREST Auth will use for generating JWTs.
                                    If the relation is a view, it is recommend that it be auto-updateable.
                                    Must be user defined. The default is postgrest.users'
                                    --refresh-relation=[rRelation] -r 'The relation where PostgREST Auth will store refresh tokens.
                                    May be defined by PostgREST Auth. The default is postgrest.refresh'
                                    --grant-issuer=[role]          -i 'As the refresh relation may be created by PostgREST Auth,
                                    the roles specified with this flag will be granted both insert and delete rights on the refresh relation.
                                    This is optional and can be done in SQL'
                                    --pass-regex=[regex]           -w 'A regular expression for validating pass properties on users.
                                    Defaults to .{6,} which is any string longer than 6 characters.
                                    The justification being that users have learned how to write a good password by now, let them use whatever they want'
                                    --jwt-expire=[time]            -e 'The relative time it takes for a JWT to expire.
                                    May be written as a number with a time unit (days, hours, minutes, seconds).
                                    Shorter times are recommended, default is 30 minutes.
                                    Important: Users may be confused at why tokens magically stop working after 30 minutes.
                                    It must be made explicitly clear in the docs why this is done'
                                    --jwt-secret=[secret]          -j 'The secret to use when encrypting JWTs. Defaults to secret'
                                    --camelcase                    -c 'Camelcases everything in the REST interface'")
                  .get_matches();
  // Get CLI Options
  let url = value_t_or_exit!(matches.value_of("url"), String);
  let port = value_t!(matches.value_of("number"), &str).unwrap_or("3001");
  let user = value_t!(matches.value_of("uRelation"), String).unwrap_or("postgrest.users".to_string());
  let refresh = value_t!(matches.value_of("rRelation"), String).unwrap_or("postgrest.refresh".to_string());
  let issuer = value_t!(matches.value_of("role"), String);
  let regex = value_t!(matches.value_of("regex"), String).unwrap_or(".{6,}".to_string());
  let expire = value_t!(matches.value_of("time"), String).unwrap_or("30m".to_string());
  let secret = value_t!(matches.value_of("secret"), String).unwrap_or("secret".to_string());
  let camelcase = matches.is_present("camelcase");
  // Start the server
  let server = Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
  }).http(String::from("localhost:") + port).unwrap();

}
