extern crate iron;
use iron::router::router!;

pub fn create_router() {
  return router!(
    // get    "/access_token"  =>
    // post   "/refresh_token" =>
    // delete "/refresh_token" =>
    // get    "/user"          =>
    // post   "/user/pass"     =>
    // post   "/users"         =>
  )
}
