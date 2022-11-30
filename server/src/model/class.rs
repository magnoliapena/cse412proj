use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ClassIdentifier {
    pub class_id: String,
}

#[derive(Deserialize)]
pub struct Class {
    pub class_uuid: String,
    pub units: String,
    pub dates: String,
    pub status: String,
    pub end_time: String,
    pub start_time: String,
    pub days: String,
    pub location: String,
    pub instructor: String,
    pub course: String,
    pub session: String,
    pub term: String,
    pub title: String,
}

impl Class {
    pub fn new(
        class_uuid: String,
        units: String,
        dates: String,
        status: String,
        end_time: String,
        start_time: String,
        days: String,
        location: String,
        instructor: String,
        course: String,
        session: String,
        term: String,
        title: String,
    ) -> Class {
        Class {
            class_uuid,
            units,
            dates,
            status,
            end_time,
            start_time,
            days,
            location,
            instructor,
            course,
            session,
            term,
            title,
        }
    }
}
