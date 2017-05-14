extern crate hyper;
extern crate hyper_native_tls;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

static URL: &'static str = "http://api.openaq.org";

pub mod json {
    use hyper::Client;
    use hyper::net::HttpsConnector;
    use hyper_native_tls::NativeTlsClient;
    use std::io::Read;

    use super::*;

    pub mod requests {
        pub const CITIES: &str = "/v1/cities";
        pub const PARAMETERS: &str = "/v1/parameters";
    }

    pub struct GetOpts {
        pub query: Option<String>,
    }

    pub fn get_cities_query(opts: GetCitiesQueryOpts) -> String {
        return String::from("page=") + &opts.page.to_string() + "&limit=" +
               &opts.limit.to_string() + "&country=" + opts.country;
    }

    pub fn get(req: &str, opts: Option<GetOpts>) -> String {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let full_url = match &opts {
            &None => String::from(URL) + req,
            &Some(ref o) => {
                match &o.query {
                    &None => String::from(URL) + req,
                    &Some(ref q) => String::from(URL) + req + "?" + q,
                }
            }
        };

        let mut res = client.get(&full_url).send().unwrap();

        assert_eq!(res.status, hyper::Ok);
        let mut s = String::new();
        res.read_to_string(&mut s).unwrap();

        return s;
    }
}

pub struct GetCitiesQueryOpts<'a> {
    pub page: u32,
    pub limit: u32,
    pub country: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    pub count: u32,
    pub country: String,
    pub locations: u32,
    #[serde(rename = "city")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub description: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "preferredUnit")]
    pub preferred_unit: String,
}

macro_rules! extract_results {
    ($T: ty, $endpoint: expr, $opts: expr) => {
        #[derive(Debug, Serialize, Deserialize)]
        struct RequestResult {
            results: Vec<$T>,
        }
        let result = json::get($endpoint, $opts);
        let v: RequestResult = serde_json::from_str(&result).unwrap();

        return v.results;
    };
}

pub fn get_cities(cities_query_opts: Option<GetCitiesQueryOpts>) -> Vec<City> {
    let get_opts = match cities_query_opts {
        None => None,
        Some(o) => {
            let cities_query = json::get_cities_query(o);
            let opts = json::GetOpts { query: Some(cities_query) };
            Some(opts)
        }
    };

    extract_results!(City, json::requests::CITIES, get_opts);
}

pub fn get_parameters() -> Vec<Parameter> {
    extract_results!(Parameter, json::requests::PARAMETERS, None);
}
