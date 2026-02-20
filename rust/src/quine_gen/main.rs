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
    buf
}

// quine generator (SVG-aware)
fn gen_quine() -> String {
    const TABLE: &[u8] = b"+-.<>[]";

    let header = ">>>".to_owned();

    // 1️⃣ Delta code for SVG prefix (full ASCII)
    let svg_header_delta = gen_printer(SVG_PREFIX);
    let svg_footer_delta = gen_printer(SVG_SUFFIX);

    // 2️⃣ Build footer template: inject SVG header delta at start
    let mut footer_templ = String::new();
    footer_templ.push_str(&svg_header_delta);
    footer_templ.push_str("
        [<]
        <+7[>+9<-]>-...
        >[
            <.-19>
            [-<.<+>>]
            <+19[->+<]
            >>
        ]
        <<<[<]
        >[
            -[-[-[-[-[-[-<+2>]<+29>]<+2>]<+14>]<+1>]<+2>]<<+5[>+9<-]>--
            .[-]>>
        ]
    ");
    footer_templ.push_str(&svg_footer_delta);

    // 3️⃣ Expand numbers in template to repeated characters
    let mut footer = String::new();
    let mut cnt = 0;
    for ch in footer_templ.chars() {
        if ch.is_ascii_digit() {
            cnt = cnt * 10 + ch.to_digit(10).unwrap();
        } else {
            if cnt != 0 {
                let prev = footer.chars().next_back().unwrap();
                for _ in 0..(cnt - 1) {
                    footer.push(prev);
                }
                cnt = 0;
            }
            if !ch.is_ascii_whitespace() {
                footer.push(ch);
            }
        }
    }
    assert_eq!(cnt, 0);

    // 4️⃣ Encode memory via 7-char table (quinerized BF)
    let mut data = String::new();
    for ch in footer.chars() {
        let idx = TABLE
            .iter()
            .enumerate()
            .find(|(_, &b)| b == ch as u8)
            .unwrap();
        data.push('>');
        for _ in 0..(idx.0 + 1) {
            data.push('+');
        }
    }

    // 5️⃣ Final program
    header + &data + &footer + &gen_printer(SVG_SUFFIX)
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
