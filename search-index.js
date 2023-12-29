var searchIndex = new Map(JSON.parse('[\
["qrc",{"doc":"A Rust library for generating and manipulating QR code …","t":"FNQQNNNNNNNNQNQNQNQONNNNNNNNNNNNNCNNQNQQNQNQNNNNNNNNN","n":["QRCode","add_image_watermark","add_image_watermark","batch_generate_qr","batch_generate_qr_codes","borrow","borrow_mut","clone","clone_into","cmp","colorize","combine_qr_codes","combine_qr_codes","compress_data","compress_data_macro","create_dynamic","create_dynamic_qr","create_multilanguage","create_multilanguage_qr","data","default","deref","deref_mut","drop","eq","fmt","from","from_bytes","from_string","get_encoding_format","hash","init","into","macros","new","overlay_image","overlay_image","partial_cmp","qr_code","qr_code_to","resize","resize","set_encoding_format","set_encoding_format","to_gif","to_jpg","to_owned","to_png","to_qrcode","to_svg","try_from","try_into","type_id"],"q":[[0,"qrc"],[53,"image::buffer_"],[54,"alloc::string"],[55,"alloc::vec"],[56,"core::cmp"],[57,"image::color"],[58,"core::result"],[59,"std::collections::hash::map"],[60,"core::fmt"],[61,"core::fmt"],[62,"core::option"],[63,"image::buffer_"],[64,"core::any"]],"d":["Represents a QR code containing data.","Adds a watermark image to the QR code.","Macro to add a watermark image to a QR code.","Generates multiple QR codes in one operation.","Generates a batch of QR codes from a vector of data …","","","","","","Colorizes the QR code with the specified color.","Combines multiple QR codes into a single larger QR code.","Combines multiple QR codes into a single QR code.","Compresses the provided data string using Zlib compression.","Compresses data before encoding it into a QR code.","Generates a dynamic QR code that can be updated after …","Generates a dynamic QR code, which can be updated after …","Creates a multilingual QR code based on a map of language …","Generates QR codes with multi-language support.","The <code>data</code> field holds the data to be encoded in the QR code.","","","","","","","Returns the argument unchanged.","Creates a new QRCode structure from a vector of bytes.","The <code>from_string</code> method creates a new instance of the QRCode","Retrieves the encoding format of the QR code.","","","Calls <code>U::from(self)</code>.","The <code>macros</code> module contains functions for generating macros.","Creates a new <code>QRCode</code> instance with the given data.","Overlays an image on top of the QR code.","Overlays an image (e.g., a logo) at the center of the QR …","","Macro to create a new QR code from the given data.","Macro to create a QR code in a specified format with a …","Resizes the QR code image to the specified width and …","Sets the size of the QR code.","Sets the encoding format of the QR code.","Sets the encoding format for the data in a QR code.","Converts the QRCode structure to a GIF image.","Converts the QRCode structure to a JPG image.","","Converts the QRCode structure to a PNG image.","Converts the QRCode structure to a QrCode structure.","Converts the QRCode structure to an SVG image.","","",""],"i":[0,5,0,0,5,5,5,5,5,5,5,5,0,5,0,5,0,5,0,5,5,5,5,5,5,5,5,5,5,5,5,5,5,0,5,5,0,5,0,0,5,0,5,0,5,5,5,5,5,5,5,5,5],"f":[0,[[1,1],2],0,0,[[[4,[3]]],[[4,[5]]]],[-1,-2,[],[]],[-1,-2,[],[]],[5,5],[[-1,-2],2,[],[]],[[5,5],6],[[5,[8,[7]]],1],[[[4,[5]]],[[10,[5,9]]]],0,[9,[[4,[7]]]],0,[9,5],0,[[[11,[3,3]]],5],0,0,[[],5],[12,-1,[]],[12,-1,[]],[12,2],[[5,5],13],[[5,14],15],[-1,-1,[]],[[[4,[7]]],5],[3,5],[5,9],[[5,-1],2,16],[[],12],[-1,-2,[],[]],0,[[[4,[7]]],5],[[5,1],1],0,[[5,5],[[17,[6]]]],0,0,[[5,18,18],1],0,[[5,9],[[10,[5,9]]]],0,[[5,18],[[19,[[8,[7]],[4,[7]]]]]],[[5,18],[[19,[[8,[7]],[4,[7]]]]]],[-1,-2,[],[]],[[5,18],[[19,[[8,[7]],[4,[7]]]]]],[5,20],[[5,18],3],[-1,[[10,[-2]]],[],[]],[-1,[[10,[-2]]],[],[]],[-1,21,[]]],"c":[],"p":[[8,"RgbaImage",53],[1,"tuple"],[5,"String",54],[5,"Vec",55],[5,"QRCode",0],[6,"Ordering",56],[1,"u8"],[5,"Rgba",57],[1,"str"],[6,"Result",58],[5,"HashMap",59],[1,"usize"],[1,"bool"],[5,"Formatter",60],[8,"Result",60],[10,"Hasher",61],[6,"Option",62],[1,"u32"],[5,"ImageBuffer",53],[5,"QrCode",63],[5,"TypeId",64]],"b":[]}],\
["xtask",{"doc":"This is the main entry point for the xtask crate.","t":"H","n":["main"],"q":[[0,"xtask"],[1,"anyhow"],[2,"core::result"]],"d":[""],"i":[0],"f":[[[],[[3,[1,2]]]]],"c":[],"p":[[1,"tuple"],[5,"Error",1],[6,"Result",2]],"b":[]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
