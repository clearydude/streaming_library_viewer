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
        // data.StandardCollection.containers[4..12].set.refId
        let data = reqwest::blocking::get(HOME_URL).unwrap().text().unwrap();
        let json: Value = serde_json::from_str(&data).unwrap();
        let ref_id = &json["data"]["StandardCollection"]["containers"][4]["set"]["refId"];

        //        println!("json: {}", json);
        println!("ref id: {:#?}", ref_id);

        let data = reqwest::blocking::get(&get_ref_url(ref_id.as_str().unwrap()))
            .unwrap()
            .text()
            .unwrap();
        let json: Value = serde_json::from_str(&data).unwrap();
        let title = &json["data"]["CuratedSet"]["items"][9]["text"]["title"]["full"]["series"]
            ["default"]["content"];

        println!("title {:?}", title);
    }

    #[test]
    fn correctly_interpolates_ref_id() {
        let ref_id = "abc664";

        assert_eq!(format!("{}{}.json", REF_URL, ref_id), get_ref_url(ref_id))
    }
}
