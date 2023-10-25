use super::dtype::DataType;

struct Table{
    name: String,
}

struct ForeignKey{
    table: String,
    column: String,
}

struct Column{
    d_type: DataType,
    unique: bool,
    null: bool,
    pk: bool,
    auto: bool,
    fk: Option<ForeignKey>,

}
