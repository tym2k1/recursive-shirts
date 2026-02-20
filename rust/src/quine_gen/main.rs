use brainfuck::{program::Program, tape}; // brainfuck = "0.2.1"

// SVG wrappers
const SVG_PREFIX: &str = r#"<svg font-family="monospace" font-size="3" style="white-space:pre;line-height:3"><text y="3"><![CDATA[
"#;

const SVG_SUFFIX: &str = r#"
]]></text></svg>"#;

// naive BF printer: generates + / - delta code per byte
fn gen_printer(s: &str) -> String {
    assert!(!s.is_empty());
    let s = s.as_bytes();
    let mut buf = "+".repeat(s[0] as usize) + ".";
    for (&prev, &cur) in s.iter().zip(&s[1..]) {
        if prev < cur {
            buf += &"+".repeat((cur - prev) as usize);
        } else if cur < prev {
            buf += &"-".repeat((prev - cur) as usize);
        }
        buf += ".";
    }
    buf += "[-]";
    buf
}

// quine generator (SVG-aware)
fn gen_quine() -> String {
    const SHIFT: usize = 3;

    // Pointer shift for quine logic
    let header = ">".repeat(SHIFT);

    // Footer: rewinds and prints memory
    let footer = {
        let rewind = "<[<]".to_owned();
        let header_printer = gen_printer(&header);
        let data_duper = format!(
            ">[<{}>[-<.<+>>]<{}.[-]>>]<<<[<]",
            "+".repeat('+' as usize),
            "+".repeat('>' as usize - '+' as usize),
        );
        let data_printer = ">[.>]".to_owned();
        rewind + &header_printer + &data_duper + &data_printer
    };

    // Encode memory: header_printer logic + footer + SVG suffix
    let mut data = String::new();
    let encoded_target = gen_printer(SVG_PREFIX) + &footer + SVG_SUFFIX;
    for b in encoded_target.bytes() {
        data += &"+".repeat(b as usize);
        data += ">";
    }

    // Build final program
    let mut final_program = String::new();
    final_program += &header;                  // pointer shift
    final_program += &data;                    // memory containing: header printer + footer + suffix
    final_program += &gen_printer(SVG_PREFIX); // print header at runtime
    final_program += &footer;                  // print memory

    final_program
}

// run BF and capture output
fn run_brainfuck(src: &str) -> String {
    let mut inp: &[u8] = &[];
    let mut oup: Vec<u8> = Vec::new();
    let program = Program::parse(&src).unwrap();
    brainfuck::Interpreter::<tape::VecTape>::new(program, &mut inp, &mut oup)
        .run()
        .unwrap();
    String::from_utf8(oup).unwrap()
}

// assert quine correctness
fn assert_quine(src: &str) {
    assert_eq!(src, run_brainfuck(src));
}

fn main() {
    let quine = gen_quine();
    // assert_quine(&quine);
    print!("{}", quine);
}
