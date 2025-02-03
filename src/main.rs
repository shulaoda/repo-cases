use std::env;

use ignore::overrides::OverrideBuilder;

fn main() {
    let cwd = env::current_dir().unwrap().join("src");
    let mut walker = ignore::WalkBuilder::new(&cwd);

    let mut builder = OverrideBuilder::new(&cwd);

    builder.add("tests/ignore/**").unwrap();
    builder.add("!tests/**").unwrap();

    walker.overrides(builder.build().unwrap());

    let walker = walker.build();
    
    for item in walker {
        println!("{:?}", item);
    }
}