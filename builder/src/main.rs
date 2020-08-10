type Definition = String;
type DefinitionList = Vec<Definition>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Kind {
    Property,
    Type,
}

struct Item {
    name: String,
    kind: Kind,
    value: Definition,
    definitions: Option<DefinitionList>,
}

fn main() {}
