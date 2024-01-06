pub mod musiums {
    use reqwest::Url;

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub struct Museum {
        pub id: String,
        pub title: String,
        pub url: String,
    }
    #[allow(dead_code)]
    impl Museum {
        pub fn into_url(&self) -> Url {
            format!("https://na.iiivega.com/api/search-result/drawer/format-groups/{}/locations?tab=Museum%20Pass", self.id).parse().unwrap()
        }
    }

    pub fn get_museums() -> Vec<Museum> {
        return vec![
            Museum {
                id: "8c8127e9-7ff5-4bb1-8127-e97ff5abb1ef".to_string(),
                title: "Black Police Precinct & Courthouse Museum".to_string(),
                url: "https://historicalblackprecinct.org/".to_string(),
            },
            Museum {
                id: "abe1f830-b9f1-4f94-a1f8-30b9f13f948a".to_string(),
                title: "HistoryMiami".to_string(),
                url: "http://www.historymiami.org".to_string(),
            },
            Museum {
                id: "090a7a3c-6ba4-458a-8a7a-3c6ba4558a72".to_string(),
                title: "Zoo Miami".to_string(),
                url: "http://www.miamimetrozoo.com/".to_string(),
            },
            Museum {
                id: "3feb985f-4e4b-4d06-ab98-5f4e4bcd0634".to_string(),
                title: "Museum of Graffiti".to_string(),
                url: "http://museumofgraffiti.com/".to_string(),
            },
            Museum {
                id: "65f3092e-a82b-466c-b309-2ea82b266c9f".to_string(),
                title: "The Bass".to_string(),
                url: "http://www.thebass.org/".to_string(),
            },
            Museum {
                id: "7a6a43b5-37d2-4e9e-aa43-b537d2ae9e1d".to_string(),
                title: "Perez Art Museum Miami".to_string(),
                url: "http://www.pamm.org".to_string(),
            },
            Museum {
                id: "83332190-cbea-4de8-b321-90cbeafde82d".to_string(),
                title: "The Fruit and Spice Park".to_string(),
                url: "http://redlandfruitandspice.com".to_string(),
            },
            Museum {
                id: "0603917f-9ddf-4c0b-8391-7f9ddf5c0b62".to_string(),
                title: "The Coral Gables Museum".to_string(),
                url: "http://coralgablesmuseum.org/".to_string(),
            },
            Museum {
                id: "22627b49-3888-4d4b-a27b-4938889d4b0b".to_string(),
                title: "Phillip and Patricia Frost Museum of Science".to_string(),
                url: "http://www.frostscience.org/".to_string(),
            },
            Museum {
                id: "90fcd072-d05f-4575-bcd0-72d05f357563".to_string(),
                title: "Miami Children's Museum".to_string(),
                url: "http://www.miamichildrensmuseum.org/".to_string(),
            },
        ];
    }
}
