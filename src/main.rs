extern crate docopt;
extern crate reqwest;

use std::env::args;
use std::collections::HashMap;
use std::io::Read;
use docopt::Docopt;

const USAGE: &'static str = "
Usage: polysync-repo-lister <user> <passkey>
       polysync-repo-lister (--help | --version)
";

const REPO_URL: &'static str = "https://api.bitbucket.org/2.0/repositories/PolySync";


fn get_repo_json(user: &str,
                 passkey: &str)
                 -> Result<HashMap<String, String>, reqwest::Error> {
    let mut request = reqwest::Url::parse(REPO_URL)?;

    request.set_username(user);

    request.set_password(Some(passkey));

    let mut response = reqwest::get(request)?;

    let mut text = String::new();

    response.read_to_string(&mut text);

    println!("{}", text);

    response.json()
}


fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|d| d.argv(args().into_iter()).parse())
        .unwrap_or_else(|e| e.exit());

    let repo_json = get_repo_json(&args.get_str("<user>"), &args.get_str("<passkey>"));

    println!("{:?}", repo_json);
}
