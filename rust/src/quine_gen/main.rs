mod string_to_brainfuck;
use string_to_brainfuck::autotune;

use brainfuck::{program::Program, tape}; // brainfuck = "0.2.1"

// SVG wrappers
const SVG_PREFIX: &str = r#"<svg font-family="monospace" font-size="3" style="white-space:pre;line-height:3"><text y="3"><![CDATA[
"#;

const SVG_SUFFIX: &str = r#"
]]></text></svg>"#;

fn split_printer_at(code: &str, count: usize) -> (String, String) {
    let mut printed = 0;
    let mut split_index = 0;

    for (i, ch) in code.char_indices() {
        if ch == '.' {
            printed += 1;
            if printed == count {
                split_index = i + 1;
                break;
            }
        }
    }

    let first = code[..split_index].to_string();
    let rest = code[split_index..].to_string();

    (first, rest)
}

fn build_decoder_logic() -> String {
    let decoder_template = "
        [<]
        <+7[>+9<-]>
        ++++++++++++++++++++++++++++.
        ----------------------------------------------.
        ++++++++++++++++++++++++++++++++++++++++++++++++.
        -------------------------------.
        ..
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
    ";

    expand_template(decoder_template)
}

fn expand_template(template: &str) -> String {
    let mut output = String::new();
    let mut count = 0;

    for ch in template.chars() {
        if ch.is_ascii_digit() {
            count = count * 10 + ch.to_digit(10).unwrap();
        } else {
            if count != 0 {
                let prev = output.chars().next_back().unwrap();
                for _ in 0..(count - 1) {
                    output.push(prev);
                }
                count = 0;
            }

            if !ch.is_ascii_whitespace() {
                output.push(ch);
            }
        }
    }

    output
}

fn encode_with_table(source: &str, table: &[u8]) -> String {
    let mut encoded = String::new();

    for ch in source.chars() {
        let idx = table
            .iter()
            .position(|&b| b == ch as u8)
            .expect("Non-BF character in payload");

        encoded.push('>');
        encoded.push_str(&"+".repeat(idx + 1));
    }

    encoded
}

// quine generator (SVG-aware)
fn gen_quine() -> String {
    const TABLE: &[u8] = b"+-.<>[]";

    let pointer_bootstrap = "[-]>>>";
    let decoder_base = 3;

    // SVG header (strip newlines once)
    let svg_open_tag = SVG_PREFIX.replace('\n', "");
    let svg_close_tag = SVG_SUFFIX.replace('\n', "");
    let full_svg = format!("{svg_open_tag}{svg_close_tag}");

    // Optimize printer
    let (optimized_svg_printer, _, _, _cells, _final_ptr, seq) =
        autotune(&full_svg);

    // Split printer
    let (optimized_svg_prefix, optimized_svg_suffix) =
        split_printer_at(&optimized_svg_printer, svg_open_tag.len());

    let prefix_ptr = seq[svg_open_tag.len() - 1];

    // Pointer math
    let prefix_start = decoder_base + 1;
    let prefix_end = prefix_start + prefix_ptr;

    let back_to_decoder = prefix_end - decoder_base;
    let forward_to_prefix_end = prefix_end - decoder_base;

    let decoder_logic = build_decoder_logic();

    // Layout builder
    let build_body = |payload: &str| -> String {
        let mut s = String::new();

        // Encoded payload (or placeholder)
        s.push_str(payload);

        // Prefix print
        s.push('>');
        s.push_str(&optimized_svg_prefix);

        // Return to decoder
        s.push_str(&"<".repeat(back_to_decoder));
        s.push('<');

        // Decoder
        s.push_str(&decoder_logic);

        // Move back to prefix end
        s.push_str(">>");
        s.push_str(&">".repeat(forward_to_prefix_end));

        // Suffix print
        s.push_str(&optimized_svg_suffix);

        // This makes also the final svg a quine
        s.push_str("[-]");

        s
    };

    // build payload source using same structure
    let payload_source = build_body("");

    // Encode it
    let encoded_payload = encode_with_table(&payload_source, TABLE);

    // Final program = bootstrap + real body
    let mut program = String::new();
    program.push_str(pointer_bootstrap);
    program.push_str(&build_body(&encoded_payload));

    program
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
