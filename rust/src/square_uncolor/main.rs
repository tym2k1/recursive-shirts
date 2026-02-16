use std::fs::write;fn main(){write
("shirt_print.svg",format!(r#"<svg
xmlns="http://www.w3.org/2000/svg"
font-family="monospace" width="63"
height="60" font-size="3">{}</svg>
"#,include_str!("main.rs").lines()
.enumerate().map(|(yi,xi)|format!(
r#"<text x="0" y="{}" >{}</text>"#
, 3+yi*5, xi.replace("&", "&amp;")
.replace("<", "&lt;").replace(">",
"&gt;"))).collect::<String>())).ok
();} // Made by <3< with love xoxo
