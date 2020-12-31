use reqwest::Url;

const HOME_URL: &str = "https://cd-static.bamgrid.com/dp-117731241344/home.json";
const REF_URL: &str = "https://cd-static.bamgrid.com/dp-117731241344/sets/";

fn get_ref_url(ref_id: &str) -> String {
    Url::parse(REF_URL)
        .unwrap()
        .join(&format!("{}.json", ref_id))
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::models::streaming_library_service::{get_ref_url, HOME_URL, REF_URL};
    use serde_json::Value;

    #[test]
    fn gets_all_collections() {
        let data = reqwest::blocking::get(HOME_URL).unwrap().text().unwrap();
        let json: Value = serde_json::from_str(&data).unwrap();
        let possible_collections = &json["data"]["StandardCollection"]["containers"];

        // TODO need to allow 4 again when I fix BecauseYouSet stuff
        for i in 5..possible_collections.as_array().unwrap().len() {
            let title = &possible_collections[i]["set"]["text"]["title"]["full"]["set"]["default"]
                ["content"];
            let ref_id = &possible_collections[i]["set"]["refId"];
            let ref_type = &possible_collections[i]["set"]["refType"];

            if !ref_id.is_null() {
                println!("{}", title);

                let data = reqwest::blocking::get(&get_ref_url(
                    ref_id.as_str().expect("Couldn't convert refid to string"),
                ))
                .expect("Couldn't get")
                .text()
                .expect("no text section");
                let json: Value = serde_json::from_str(&data).expect("Couldn't get json");

                let possible_titles = &json["data"][ref_type.as_str().unwrap()]["items"];

                for i in 0..possible_titles
                    .as_array()
                    .expect("Couldn't get array")
                    .len()
                {
                    // ugh might be program instead of series
                    let title = &possible_titles[i]["text"]["title"]["full"]["series"]["default"]
                        ["content"];
                    if !title.is_null() {
                        println!("\t{}", title);
                    } else {
                        let title = &possible_titles[i]["text"]["title"]["full"]["program"]
                            ["default"]["content"];
                        if !title.is_null() {
                            println!("\t{}", title);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn correctly_interpolates_ref_id() {
        let ref_id = "abc664";

        assert_eq!(format!("{}{}.json", REF_URL, ref_id), get_ref_url(ref_id))
    }
}
