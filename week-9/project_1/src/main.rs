use std::io::Write;

fn main() {
    let lager = vec!["33 Export", "Desperados", "Goldberg" ,"Gulder", "Heineken", "Star"];
    let stout = vec!["Larger", "Turbo King", "Williams","-","-", "-"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz", "-", "-"];

    let mut file = std::fs::File::create("Nigerian_Breweries_Plc.txt").expect("failed to create file");
    
    
    file.write_all("LAGER".as_bytes()).expect("Failed to write");
    file.write_all("                    |".as_bytes()).expect("Failed to write");
    file.write_all("STOUT".as_bytes()).expect("Failed to write");
    file.write_all("                    |".as_bytes()).expect("Failed to write");
    file.write_all("NON_ALCOHOLIC\n".as_bytes()).expect("Failed to write");




     for i in 0..lager.len() {
         file.write_all( lager[i].as_bytes()).expect("failed to write");
         file.write_all("                   ".as_bytes()).expect("Failed to write");
         file.write_all(stout[i].as_bytes()).expect("failed to write");
         file.write_all("                   ".as_bytes()).expect("Failed to write ");
         file.write_all(non_alcoholic[i].as_bytes()).expect("failed to write");


     }
   
}
