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

    // 1️⃣ bootstrap pointer to cell 3
    let pointer_bootstrap = ">>>";

    let decoder_base = 3;

    // 2️⃣ SVG open/close (no newlines)
    let svg_open_tag = SVG_PREFIX.replace('\n', "");
    let svg_close_tag = SVG_SUFFIX.replace('\n', "");
    let full_svg = format!("{}{}", svg_open_tag, svg_close_tag);

    // 3️⃣ Optimize full SVG printer
    let (optimized_svg_printer, _, _, cells, _final_ptr, seq) =
        autotune(&full_svg);

    // 4️⃣ Split at open tag boundary
    let (optimized_svg_prefix, optimized_svg_suffix) =
        split_printer_at(&optimized_svg_printer, svg_open_tag.len());

    let prefix_ptr = seq[svg_open_tag.len() - 1];

    // POINTER MATH
    let shift_for_prefix = 1;

    let prefix_start = decoder_base + shift_for_prefix;

    let prefix_end = prefix_start + prefix_ptr;

    // How far to go back after prefix?
    let back_to_decoder = prefix_end - decoder_base;

    // How far to return later?
    let forward_to_prefix_end = prefix_end - decoder_base;

    // ─────────────────────────────────────
    // Build decoder logic
    // ─────────────────────────────────────
    let decoder_logic = build_decoder_logic();

    // Build encoded payload
    let payload_source = format!(
        "{}{}{}",
        optimized_svg_prefix,
        decoder_logic,
        optimized_svg_suffix
    );

    let encoded_payload = encode_with_table(&payload_source, TABLE);

    // Final layout
    let mut program = String::new();

    // Bootstrap to cell 3
    program.push_str(pointer_bootstrap);
    //program.push_str("\n");

    // Encode payload
    program.push_str(&encoded_payload);
    //program.push_str("\n");

    // Move one right for SVG prefix printing
    program.push('>');

    // Print SVG prefix
    program.push_str(&optimized_svg_prefix);
    //program.push_str("\n");

    // Return to decoder base
    program.push_str(&"<".repeat(back_to_decoder));

    // Move one left more to return to pre-print position
    program.push('<');

    // Run decoder
    program.push_str(&decoder_logic);

    // Move one right for SVG prefix printing
    program.push('>');

    // Move one right decoder ending on -1 index
    program.push('>');

    // Go back to end-of-prefix pointer
    program.push_str(&">".repeat(forward_to_prefix_end));

    // Print SVG suffix
    program.push_str(&optimized_svg_suffix);

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
