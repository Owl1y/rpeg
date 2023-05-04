use std::collections::HashMap;

fn main() {
<<<<<<< HEAD
    let mark_mapping = HashMap::from([
        (0xffd8, "Start of Image"),
        (0xffe0, "Application Default Header"),
        (0xffdb, "Quantization Table"),
        (0xffc0, "Start of Frame"),
        (0xffc4, "Define Huffman Table"),
        (0xffda, "Start of Scan"),
        (0xffd9, "End of Image")
=======
    
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

>>>>>>> c26d8c30dd77c4f76c7f326a2267244df6f8f6e2

    ]);

    println!("{:?}", mark_mapping);
}


