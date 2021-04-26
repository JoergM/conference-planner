#[derive(Debug, Clone)]
pub struct ScheduleEntry {
    pub id: u32,
    pub start_time: String,
    pub end_time: String,
    pub session_id: u32,
}

pub fn generate_examples() -> Vec<ScheduleEntry> {
    let entries = vec![
        ScheduleEntry {
            id: 1,
            start_time: "09:30".into(),
            end_time: "10:15".into(),
            session_id: 1,
        },
        ScheduleEntry {
            id: 2,
            start_time: "10:30".into(),
            end_time: "11:15".into(),
            session_id: 5,
        },
        ScheduleEntry {
            id: 3,
            start_time: "12:30".into(),
            end_time: "13:15".into(),
            session_id: 3,
        },
        ScheduleEntry {
            id: 4,
            start_time: "13:30".into(),
            end_time: "14:15".into(),
            session_id: 4,
        },
        ScheduleEntry {
            id: 5,
            start_time: "14:30".into(),
            end_time: "15:15".into(),
            session_id: 2,
        },
    ];

    entries
}
