extern crate openaq_client;
extern crate serde_json;

fn main() {
    // let cities_query_opts = openaq_client::GetCitiesQueryOpts {
    // country: "US",
    // limit: 200,
    // page: 4,
    // };
    // let cities_query = openaq_client::json::get_cities_query(cities_query_opts);
    // let opts = openaq_client::json::GetOpts { query: Some(cities_query) };
    // let result = openaq_client::json::get(openaq_client::json::requests::CITIES, Some(opts));
    // let v: serde_json::Value = serde_json::from_str(&result).unwrap();
    // let result_formatted = serde_json::ser::to_string_pretty(&v).unwrap();

    // println!("result: {}", result_formatted);

    let get_cities_opts = openaq_client::GetCitiesQueryOpts {
        country: "ES",
        limit: 1,
        page: 1,
    };
    let cities = openaq_client::get_cities(Some(get_cities_opts));

    println!("First city received: {:?}", cities[0])
}
