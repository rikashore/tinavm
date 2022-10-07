use lalrpop_util::lalrpop_mod;

pub mod ait;
pub mod runtime;

lalrpop_mod!(parse);

fn main() {
    let src = r#"
.quarry hello

.const 0 11
.const 1 "rika"

.func(i;i -- i) @add [
    pop 2;
    addi;
    push;
]

.func(i -- i) @square [
    pop;
    dup;
    muli;
    push;
]
"#;

    let a = parse::QuarryParser::new().parse(src);
    println!("{:?}", a.unwrap())
}
