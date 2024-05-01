//to run : cargo run -- Thomas Grinding
//this project takes args1 as 0 is the project path then takes this argument and see if it's in json file or not, if it is in json file then we will print it (print 3 obj only)
#![deny(clippy::all)]
//You can use std::env to read environment variables.
use std::env;

//define api url
const API_URL: &str =
    "https://vpic.nhtsa.dot.gov/api/vehicles/getallmanufacturers?format=json&page=2";

//type string slice
struct Manufacturer<'a> {
    //option as we would recive nth from api and some of them dont have mtr name
    name: Option<&'a str>,
    common_name: Option<&'a str>,
    country: Option<&'a str>,
}

//trait for searching for items
trait Contains {
    fn contains(&self, needle: &str) -> bool;
}

impl<'a> Contains for Manufacturer<'a> {
    fn contains(&self, needle: &str) -> bool {
        //// Use unwrap_or_default to safely handle Options and check if the fields contain the needle
        self.name.unwrap_or_default().contains(needle)
            || self.common_name.unwrap_or_default().contains(needle)
            || self.country.unwrap_or_default().contains(needle)
    }
}

//print
impl<'a> Manufacturer<'a> {
    fn descripition(&self) -> String {
        let name = self.name.unwrap_or_default();
        let common_name = self.common_name.unwrap_or_default();
        let country = self.country.unwrap_or_default();

        format!(
            "\tName: {}\n\tComman Name: {}\n\tCountry: {}",
            name, common_name, country
        )
    }
}

//turn main into tokio //and return result
// result type which can either be string slice or error
//we have to put error in a box which is reference to a heap or ()
//() void , or any type of error
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //how to read args
    //The line of code you've posted is a typical Rust snippet used to collect command-line arguments into a Vec<String>
    //collect:transforms an iterator into a collection
    let args: Vec<String> = env::args().collect();
    //args must have at least 2 args, first:actual path to application , sec : acutal argumant that user pass to our app
    //if less than 2 return errror
    if args.len() < 2 {
        println!("usage: {} <search term", args[0]);
        return Ok(());
    }

    //get keyword //ex:tyota
    let keyword = &args[1];

    //read the respond and parse it as JSON
    //ok or Err fel error handling
    //send request to our url
    //create request client with reqwest
    let client = reqwest::Client::new();
    //since it is future use await , hatb2a result
    //request builder for an HTTP GET request to the URL specified by API_URL.
    //.send(): Sends the HTTP request. send() is asynchronous and returns a future.
    //.await?: The await is used to pause the function execution until the future is resolved.
    let res = client
        .get(API_URL)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    //grab the manufacturer Jason object
    //json file have lots of keys we need to go in to results and _ means ignore the value
    //unwrap is unsafly for optionalss
    //we dont need key we need it's values .1;
    //then take it as array
    let manufacturer_json = res //json value
        .as_object() //This method converts the serde_json::Value to an Option<&Map<String, Value>>. It returns None if res is not a JSON object.
        .unwrap() //// Unwrap the Option, panics if `res` is not an object
        .iter() // Get an iterator over the Map (which contains key-value pairs)
        .find(|(key, _)| key == &"Results") // Find the pair where the key matches "Results"
        .unwrap() // Unwrap the result of find, panics if no such key is found
        .1 // Access the value part of the (key, value) pair
        .as_array() // Try to convert the value to an Option<&Vec<Value>>
        .unwrap() // Unwrap the Option, panics if the value is not an array
        .iter(); // Get an iterator over the array

    //parse all the manufacturers
    //instance of man struct
    let manufactuerers = manufacturer_json.map(|manufacturer_json| {
        //parse manufacturer_json into manufact.. objects
        // obj : map of string and value , heya kol haga fa results
        let obj = manufacturer_json.as_object().unwrap();
        let country = obj.get("Country").unwrap().as_str(); //esmaha keda fel json file //it's str in json also
        let common_name = obj.get("Mfr_CommonName").and_then(|v| v.as_str()); //Chatgpt
        let name = obj.get("Mfr_Name").unwrap().as_str(); //esmaha keda fel json file //it's str in json also

        Manufacturer {
            name,
            common_name,
            country,
        }
    });
    //collect manufacture into a vector
    //// Filtering manufacturers that contain the keyword
    // keyword: This is a string that functions as a search term.
    let found_manufacturers = manufactuerers
        .filter(|m| m.contains(keyword))
        .collect::<Vec<_>>(); // to transform the iterator into a collection

    //tell user if no manufacturers found
    if found_manufacturers.is_empty() {
        println!("no manufacturers found");
    } else {
        println!("Found {} manufacturers", found_manufacturers.len());
        for (index, man) in found_manufacturers.iter().enumerate() {
            println!("manufacturers # {}", index + 1);
            println!("{}", man.descripition());
        }
    }
    Ok(())
}
