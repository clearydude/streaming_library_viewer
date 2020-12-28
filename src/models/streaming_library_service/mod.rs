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
    use crate::models::streaming_library_service::{get_ref_url, REF_URL};

    #[test]
    fn correctly_interpolates_ref_id() {
        let ref_id = "abc664";

        assert_eq!(format!("{}{}.json", REF_URL, ref_id), get_ref_url(ref_id))
    }
}
