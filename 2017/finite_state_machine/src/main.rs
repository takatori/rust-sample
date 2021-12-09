// This is the list of states for the label parsing state machine
enum LabelParseState {
    LabelLengthOrPointer, // basically the start of the FSM
    Label(u8),   // storing length of the label
    Pointer(u8), // location of pointer in slice,
    Root,        // root is the end of the labels list, aka null
}



/// parses the chain of labels
/// this has a mux of 255 octets, with each label being less than 63.
/// all names will be stored lowercase internally.
/// This will consume the portions of the Vec witch it is reading...
pub fn parse(slice: &mut Vec<u8>) -> Result<Name, FromUtf8Error> {

    let mut state: LabelParseState = LabelParseState::LabelLengthOrPointer;
    let mut labels: Vec<String> = Vec::with_capacity(3); // www.example.com

    // assume all chars are utf-8. We're doing byte-by-byte operations,
    //   no endianess issues...
    // reserved: (1000 0000 aka 0800) && (0100 0000 aka 0400)
    // pointer: (slice == 1100 0000 aka C0), then 03FF & slice = offset
    // label: 03FF & slice = length; slice.next(length) = label
    // root: 0000
    loop {
        state = match state {
            LabelParseState::LabelLengthOrPointer => {
                // determine what the next label is
                match slice.pop() {
                    Some(0) | None => LabelParseState::Root,
                    Some(byte) if byte & 0xC0 == 0xC0 =>
                        LabelParseState::Pointer(byte & 0x3F),
                    Some(byte) if byte <= 0x3F => LabelParseState::Lable(byte),
                    _ => unimplemented!()
                }
            },
            LableParseState::Label(count) => {
                labels.push(try!(util::parse_lable(slice, count)));

                // reset to collect more data
                LabelParseState::LabelLengthOrPointer
            },
            LabelParseState::Pointer(offset) => {
                // lookup in the hashmap the label to used
                unimplemented!()
            },
            LabelParseState::Root => {
                // technically could return here...
                break;
            }
        }
    }
    
    Ok(name { label: labels })    
}



fn main() {
    println!("Hello, world!");
}
