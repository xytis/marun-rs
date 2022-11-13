# Target
```rust
use marun::{Marshal, Unmarshal};

#[derive(Marshal(xml), Unmarshal(json))]
#[marun(xml(tag="simple"), json(key="simple"))]
struct Simple {
  #[marun(xml(attr="value_a"), json(key="a", value=string))]
  a: String,
  #[marun(xml(val=cdata)), json(key="b", value=number)]
  b: i64,
}

fn main() {
    let input = r#"{ "a": "val_a", "b": 1 } "#;
    let s: Simple = marun::json::from_str(&input);
    let output = marun::xml::to_str(&s);
    println!(output);
    // <simple><value_a>val_a</value_a><b><![CDATA[1]]></b></simple>
}
```
