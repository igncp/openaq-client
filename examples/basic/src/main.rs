extern crate openaq_client;
extern crate serde_json;

fn format_json_str(result: &String) -> String {
    let v: serde_json::Value = serde_json::from_str(&result).unwrap();
    let result_formatted = serde_json::ser::to_string_pretty(&v).unwrap();

    return result_formatted;
}

macro_rules! print_formatted_entities_query {
    ($query: expr, $endpoint: expr, $log_text: expr) => {
        let opts = openaq_client::json::GetOpts { query: $query };
        let result = openaq_client::json::get($endpoint, Some(opts));
        let result_formatted = format_json_str(&result);

        println!($log_text, result_formatted);
    };
}

macro_rules! print_formatted_entities {
    ($endpoint: expr, $log_text: expr) => {
        print_formatted_entities_query!(None, $endpoint, $log_text);
    };
}

macro_rules! print_formatted_entities_with_opts {
    ($opts: expr, $get_query: expr, $endpoint: expr, $log_text: expr) => {
        let entities_query_opts = $opts;
        let query = $get_query(entities_query_opts);
        print_formatted_entities_query!(Some(query), $endpoint, $log_text);
    };
}

#[allow(dead_code)]
fn print_formatted_cities_json() {
    print_formatted_entities_with_opts!(openaq_client::GetCitiesQueryOpts {
                                            country: "US",
                                            limit: 200,
                                            page: 4,
                                        },
                                        openaq_client::json::get_cities_query,
                                        openaq_client::json::requests::CITIES,
                                        "cities: {}");
}

#[allow(dead_code)]
fn print_formatted_countries_json() {
    print_formatted_entities_with_opts!(openaq_client::GetCountriesQueryOpts { limit: 5, page: 2 },
                                        openaq_client::json::get_countries_query,
                                        openaq_client::json::requests::COUNTRIES,
                                        "countries: {}");
}

#[allow(dead_code)]
fn print_formatted_fetches_json() {
    print_formatted_entities_with_opts!(openaq_client::GetFetchesQueryOpts { limit: 5, page: 2 },
                                        openaq_client::json::get_fetches_query,
                                        openaq_client::json::requests::FETCHES,
                                        "Fetches: {}");
}

#[allow(dead_code)]
fn print_formatted_parameters_json() {
    print_formatted_entities!(openaq_client::json::requests::PARAMETERS, "parameters: {}");
}

#[allow(dead_code)]
fn print_city() {
    let get_cities_opts = openaq_client::GetCitiesQueryOpts {
        country: "ES",
        limit: 1,
        page: 1,
    };
    let cities = openaq_client::get_cities(Some(get_cities_opts));

    println!("First city received: {:?}", cities[0])
}

#[allow(dead_code)]
fn print_country() {
    let get_countries_opts = openaq_client::GetCountriesQueryOpts { limit: 1, page: 1 };
    let countries = openaq_client::get_countries(Some(get_countries_opts));

    println!("First country received: {:?}", countries[0])
}

#[allow(dead_code)]
fn print_parameter() {
    let parameters = openaq_client::get_parameters();

    println!("Second parameter received: {:?}", parameters[1])
}

fn main() {
    // Uncomment the lines you want to test:

    // print_formatted_cities_json();
    // print_formatted_countries_json();
    // print_formatted_fetches_json();
    // print_formatted_parameters_json();

    // print_city()
    // print_country()
    // print_fetch()
    // print_parameter()
}
