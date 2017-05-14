extern crate openaq_client;
extern crate serde_json;

fn format_json_str(result: &String) -> String {
    let v: serde_json::Value = serde_json::from_str(&result).unwrap();
    let result_formatted = serde_json::ser::to_string_pretty(&v).unwrap();

    return result_formatted;
}

#[allow(dead_code)]
fn print_formatted_cities_json() {
    let cities_query_opts = openaq_client::GetCitiesQueryOpts {
        country: "US",
        limit: 200,
        page: 4,
    };
    let cities_query = openaq_client::json::get_cities_query(cities_query_opts);
    let opts = openaq_client::json::GetOpts { query: Some(cities_query) };
    let result = openaq_client::json::get(openaq_client::json::requests::CITIES, Some(opts));
    let result_formatted = format_json_str(&result);

    println!("cities: {}", result_formatted);
}

#[allow(dead_code)]
fn print_formatted_parameters_json() {
    let opts = openaq_client::json::GetOpts { query: None };
    let result = openaq_client::json::get(openaq_client::json::requests::PARAMETERS, Some(opts));
    let result_formatted = format_json_str(&result);

    println!("parameters: {}", result_formatted);
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
fn print_parameter() {
    let parameters = openaq_client::get_parameters();

    println!("Second parameter received: {:?}", parameters[1])
}

fn main() {
    // Uncomment the lines you want to test:

    // print_formatted_cities_json();
    // print_formatted_parameters_json();

    // print_city()
    // print_parameter()
}
