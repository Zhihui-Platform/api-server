use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deduction {
    deductionid: String, // The id of the deduction.
    person: u32,         // The person who get the deduction.
    reason: String,      // The reason of the deduction, following **deduction reason id**.
    deduction: f32,      // The deduction of the deduction.
    time: usize,         // The time of the deduction.
    deductor: u32,       // The person who deduct the deduction.
    desc: String,        // The description of the deduction.
}
impl Deduction {
    pub fn new(
        deductionid: String,
        person: u32,
        reason: String,
        deduction: f32,
        time: usize,
        deductor: u32,
        desc: String,
    ) -> Deduction {
        Deduction {
            deductionid,
            person,
            reason,
            deduction,
            time,
            deductor,
            desc,
        }
    }
    pub fn register(&self) {
        // Register a deduction.
        use mongodb::{sync::Client, bson::doc};
        let collection = Client::with_uri_str("http://localhost:27017")
            .expect("Failed to initialize standalone client.")
            .database("test")
            .collection::<Deduction>("deduction");
        let result = collection.find_one(doc! {
          "deductionId": &self.deductionid,
        }, None);
        println!("{:?}", result);
    }
}
