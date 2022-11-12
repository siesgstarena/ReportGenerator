use mongodb::bson::{Bson};
use docx_rs::*;
pub fn table(users : &Vec<Bson>) -> Result<(), DocxError>{
    let path = std::path::Path::new("./table.docx");
    let file = match std::fs::File::create(&path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to create file: {:?}", e),
    };
    let mut table = Table::new(vec![
        TableRow::new(vec![
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Sr.No").fonts(RunFonts::new().ascii("Calibri (Body)")).size(28))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Name").fonts(RunFonts::new().ascii("Calibri (Body)")).size(28))),
        ]),
    ]);
    // Adding rows to the table
    for (i,user) in users.iter().enumerate(){
        table = table.add_row(TableRow::new(vec![
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text((i+1).to_string().as_str()).fonts(RunFonts::new().ascii("Calibri (Body)")).size(28))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(user.as_str().unwrap()).fonts(RunFonts::new().ascii("Calibri (Body)")).size(28))),
        ]));
    } 
    match Docx::new().add_table(table).build().pack(file) {
        Ok(_) => println!("Successfully created table.docx"),
        Err(e) => panic!("Failed to create table.docx: {:?}", e),
    }; 

    Ok(())
    
}