use pdxlua::parser::parse_root_statements;

mod unittests;

#[test]
pub fn full_test() {
    let text = std::fs::read_to_string("file.lua").unwrap();

    let res = parse_root_statements(&text).unwrap();

    let _ = std::fs::write("dump.json", serde_json::to_string_pretty(&res.1).unwrap());
}
