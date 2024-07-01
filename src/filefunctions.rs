use xlsxwriter::Workbook;

use crate::error_manager::Errornumber;


pub fn create_excel(students_vec: &mut VecDeque<Students>)->Result<() , Errornumber> {
    
    let workbook = Workbook::new(file_path.to_str().unwrap());
            let mut sheet = workbook.add_worksheet(Some("Sheet1")).map_err(Errornumber::from)?;

            // Escribir encabezados
            let headers = vec!["ID", "Name", "Age", "Courses"];
            for (col, header) in headers.iter().enumerate() {
                sheet.write_string(0, col as u32, header, None).map_err(Errornumber::from)?;
            }

            // Escribir datos de estudiantes
            for (row, student) in students_vec.iter().enumerate() {
                sheet.write_number((row + 1) as u32, 0, student.id as f64, None).map_err(Errornumber::from)?;
                sheet.write_string((row + 1) as u32, 1, &student.name, None).map_err(Errornumber::from)?;
                sheet.write_number((row + 1) as u32, 2, student.age as f64, None).map_err(Errornumber::from)?;
                sheet.write_string((row + 1) as u32, 3, &student.courses.join(", "), None).map_err(Errornumber::from)?;
            }

            workbook.close().map_err(Errornumber::from)?;
            Ok(())
        }  
    
