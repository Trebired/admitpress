use super::*;
use std::io::Read;
use zip::ZipArchive;

#[test]
fn exports_selected_days_to_xlsx_and_ods() {
    let mut service = service();
    let local_date = today_local();
    let empty_date = "2000-01-01".to_owned();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, &local_date);
    service
    .print_one(&ticket.id, &summary.subtickets[0].id, &local_date, None, None)
    .expect("ticket should print");

    let dates = vec![local_date.clone(), empty_date.clone()];
    let xlsx_path = service.data_dir().join("exports").join("days.xlsx");
    service
    .export_day_sales(&ticket.id, &dates, &xlsx_path.to_string_lossy())
    .expect("xlsx should export");
    let xlsx_sheet = read_zip_entry(&xlsx_path, "xl/worksheets/sheet1.xml");
    assert!(xlsx_sheet.contains(&local_date));
    assert!(xlsx_sheet.contains(&empty_date));
    assert!(xlsx_sheet.contains("1x 10"));
    assert!(xlsx_sheet.contains("No sales"));

    let ods_path = service.data_dir().join("exports").join("days.ods");
    service
    .export_day_sales(&ticket.id, &dates, &ods_path.to_string_lossy())
    .expect("ods should export");
    let ods_sheet = read_zip_entry(&ods_path, "content.xml");
    assert!(ods_sheet.contains(&local_date));
    assert!(ods_sheet.contains("1x 10"));
}

fn read_zip_entry(path: &Path, name: &str) -> String {
    let file = fs::File::open(path).expect("spreadsheet should open");
    let mut archive = ZipArchive::new(file).expect("spreadsheet should be a zip file");
    let mut entry = archive.by_name(name).expect("entry should exist");
    let mut contents = String::new();
    entry
    .read_to_string(&mut contents)
    .expect("entry should be text");
    contents
}
