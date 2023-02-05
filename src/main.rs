use std::collections::HashMap;

fn main() {
    
    let marker_mapping = HashMap::from([
        (0xffd8, "Start of Image"),
        (0xffe0, "Application of default header"),
        (0xffdb, "Quantization table"),
        (0xffc0, "Start of frame"),
        (0xffc4, "Define huffman table"),
        (0xffda, "Start of scan"),
        (0xffd9, "end of image"),
    ]);
    
    println!("{:?}", marker_mapping); 


}


