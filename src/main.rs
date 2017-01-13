extern crate docopt;
extern crate curl;
extern crate json;

use std::env::args;
use std::str;
use docopt::Docopt;
use json::JsonValue;
use curl::easy::Easy;


fn get_repo_pages(user: &str, passkey: &str, url: &str) -> Result<Vec<JsonValue>, json::Error> {
    let mut curl_handle = Easy::new();
    let mut pages = Vec::new();

    curl_handle.username(user).expect("Invalid username");
    curl_handle.password(passkey).expect("Invalid passkey");

    let mut next_opt: Option<String> = Some(String::from(url));

    while let Some(next) = next_opt {
        let mut page = get_repo_page(&mut curl_handle, &next)?;

        next_opt = page["next"].take_string();

        pages.push(page);
    }

    Ok(pages)
}


fn get_repo_page(curl_handle: &mut Easy, url: &str) -> Result<JsonValue, json::Error> {
    curl_handle.url(url).unwrap();

    let mut page = Vec::new();

    {
        let mut transfer = curl_handle.transfer();

        transfer.write_function(|data| {
                page.extend_from_slice(data);
                Ok(data.len())
            })
            .expect("Write function should succeed");

        transfer.perform()
            .expect("Transfer should succeed");
    }

    let json_string = str::from_utf8(&page).unwrap();

    json::parse(json_string)
}


fn get_repo_pages_info(pages: &mut Vec<JsonValue>) -> Vec<String> {
    let mut repos_info = Vec::new();

    for page in pages {
        if let JsonValue::Array(ref mut values) = page["values"] {
            for repo in values {
                repos_info.push(get_repo_info(repo));
            }
        }
    }

    repos_info
}


fn get_repo_info(repo: &mut JsonValue) -> String {

    let name = repo["name"]
        .take_string()
        .expect("Repo name should be a string");

    let description = repo["description"]
        .take_string()
        .expect("Repo description should be a string");

    // access more repo fields here, modify format string below

    format!("{}: {}\n", name, description)
}

const USAGE: &'static str = "
Usage: polysync-repo-lister <username> <passkey> <owner>
       polysync-repo-lister (--help | --version)
";


fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|d| d.argv(args().into_iter()).parse())
        .unwrap_or_else(|e| e.exit());

    let username = args.get_str("<username>");
    let passkey = args.get_str("<passkey>");
    let owner = args.get_str("<owner>");
    let url = format!("https://api.bitbucket.org/2.0/repositories/{}", owner);

    let mut pages = get_repo_pages(username, passkey, &url).expect("Failed getting repo pages");

    let repo_info: String = get_repo_pages_info(&mut pages)
        .into_iter()
        .collect();

    print!("{}", repo_info);
}
