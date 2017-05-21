extern crate hyper;
extern crate hyper_native_tls;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

static URL: &'static str = "http://api.openaq.org";

mod entities;
pub use entities::*;

pub mod json {
    use hyper::Client;
    use hyper::net::HttpsConnector;
    use hyper_native_tls::NativeTlsClient;
    use std::io::Read;

    use super::*;

    pub mod requests {
        pub const CITIES: &str = "/v1/cities";
        pub const COUNTRIES: &str = "/v1/countries";
        pub const PARAMETERS: &str = "/v1/parameters";
        pub const FETCHES: &str = "/v1/fetches";
    }

    pub struct GetOpts {
        pub query: Option<String>,
    }

    pub fn get_cities_query(opts: GetCitiesQueryOpts) -> String {
        return String::from("page=") + &opts.page.to_string() + "&limit=" +
               &opts.limit.to_string() + "&country=" + opts.country;
    }

    pub fn get_countries_query(opts: GetCountriesQueryOpts) -> String {
        return String::from("page=") + &opts.page.to_string() + "&limit=" + &opts.limit.to_string();
    }

    pub fn get_fetches_query(opts: GetFetchesQueryOpts) -> String {
        return String::from("page=") + &opts.page.to_string() + "&limit=" + &opts.limit.to_string();
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

pub struct GetCountriesQueryOpts {
    pub page: u32,
    pub limit: u32,
}

pub struct GetFetchesQueryOpts {
    pub page: u32,
    pub limit: u32,
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

macro_rules! extract_results_with_opts {
    ($get_query: expr, $T: ty, $endpoint: expr, $opts: expr) => {
        let get_opts = match $opts {
            None => None,
            Some(o) => {
                let query = $get_query(o);
                let opts = json::GetOpts { query: Some(query) };
                Some(opts)
            }
        };

        extract_results!($T, $endpoint, get_opts);
    };
}

pub fn get_cities(cities_query_opts: Option<GetCitiesQueryOpts>) -> Vec<City> {
    extract_results_with_opts!(json::get_cities_query,
                               City,
                               json::requests::CITIES,
                               cities_query_opts);
}

pub fn get_fetches(fetches_query_opts: Option<GetFetchesQueryOpts>) -> Vec<Fetch> {
    extract_results_with_opts!(json::get_fetches_query,
                               Fetch,
                               json::requests::FETCHES,
                               fetches_query_opts);
}

pub fn get_countries(countries_query_opts: Option<GetCountriesQueryOpts>) -> Vec<Country> {
    let get_opts = match countries_query_opts {
        None => None,
        Some(o) => {
            let countries_query = json::get_countries_query(o);
            let opts = json::GetOpts { query: Some(countries_query) };
            Some(opts)
        }
    };

    extract_results!(Country, json::requests::COUNTRIES, get_opts);
}

pub fn get_parameters() -> Vec<Parameter> {
    extract_results!(Parameter, json::requests::PARAMETERS, None);
}
