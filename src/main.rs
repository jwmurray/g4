use google_sheets4 as sheets4;
use hyper;
use hyper_rustls;
use oauth2::{ApplicationSecret, Authenticator, DefaultAuthenticatorDelegate, MemoryStorage};
use sheets4::Sheets;
use sheets4::ValueRange;
use sheets4::{Error, Result};
use std::default::Default;
use yup_oauth2 as oauth2;

use serde;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

// pub fn read_file(filepath: &str) -> String {
//     let file = File::open(filepath).expect("could not open file");
//     let mut buffered_reader = BufReader::new(file);
//     let mut contents = String::new();
//     let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
//         Ok(number_of_bytes) => number_of_bytes,
//         Err(_err) => 0,
//     };

//     contents
// }

fn main() {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and
    // `client_secret`, among other things.
    // let secret = read_file("client_secret.json");

    let json_file_path = Path::new("/Users/jmurray/code/rust/g4/client_secret.json");
    let mut json_file = File::open(json_file_path).expect("file not found");
    let mut contents = String::new();
    json_file.read_to_string(&mut contents);
    pub const SECRET2: &'static str = "{\"installed\":{\"client_id\":\
    \"47226532167-gaq0fkfjlu1kqqinumvpa4klkbt7oucj.apps.googleusercontent.com\",\
    \"project_id\":\"quickstart-1566671452478\",\
    \"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\
    \"token_uri\":\"https://oauth2.googleapis.com/token\",\
    \"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\",\
    \"client_secret\":\"NBHXA64gJ3wh2rdTXjcaOjsP\",\
    \"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\
    \"http://localhost\"]}}";

    println!("{:?}", contents);
    // let secret: ApplicationSecret = serde_json::from_str::<ApplicationSecret>(SECRET2);
    // serde_json::from_str(&contents).expect("Failed to read json to struct");

    let mut secret: ApplicationSecret = Default::default();
    secret.client_id =
        String::from("47226532167-gaq0fkfjlu1kqqinumvpa4klkbt7oucj.apps.googleusercontent.com");
    secret.auth_provider_x509_cert_url =
        serde::export::Option::Some(String::from("https://www.googleapis.com/oauth2/v1/certs"));
    secret.auth_uri = String::from("https://accounts.google.com/o/oauth2/auth");
    secret.project_id = serde::export::Option::Some(String::from("quickstart-1566671452478"));
    secret.token_uri = String::from("token_uri");
    secret.client_secret = String::from("NBHXA64gJ3wh2rdTXjcaOjsP");
    secret.redirect_uris = vec![
        String::from("urn:ietf:wg:oauth:2.0:oob"),
        String::from("http://localhost"),
    ];
    // serde_json::from_str(&contents);
    // let secret: ApplicationSecret = ConsoleApplicationSecret::default();

    // Instantiate the authenticator. It will choose a suitable authentication flow for you,
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let auth = Authenticator::new(
        &secret,
        DefaultAuthenticatorDelegate,
        hyper::Client::with_connector(hyper::net::HttpsConnector::new(
            hyper_rustls::TlsClient::new(),
        )),
        <MemoryStorage as Default>::default(),
        None,
    );
    let mut hub = Sheets::new(
        hyper::Client::with_connector(hyper::net::HttpsConnector::new(
            hyper_rustls::TlsClient::new(),
        )),
        auth,
    );
    // As the method needs a request, you would usually fill it with the desired information
    // into the respective structure. Some of the parts shown here might not be applicable !
    // Values shown here are possibly random and not representative !
    let mut req = ValueRange::default();

    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let result = hub
        .spreadsheets()
        .values_append(req, "spreadsheetId", "range")
        .value_input_option("justo")
        .response_value_render_option("amet.")
        .response_date_time_render_option("erat")
        .insert_data_option("labore")
        .include_values_in_response(true)
        .doit();

    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }
}
