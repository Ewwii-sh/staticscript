//! Not used
// pub fn extract_map_entries(box_content: &str) -> String {
//     if let Some(start) = box_content.find("MAP_ENTRIES: [") {
//         if let Some(end) = box_content.find("]; CHILDREN: [") {
//             return box_content[start + 13..end].trim().to_string();
//         }
//     }
//     String::new()
// }

// pub fn extract_children(box_content: &str) -> String {
//     if let Some(start) = box_content.find("CHILDREN: [") {
//         if let Some(end) = box_content.rfind("]") {
//             return box_content[start + 11..end].trim().to_string();
//         }
//     }
//     String::new()
// }
