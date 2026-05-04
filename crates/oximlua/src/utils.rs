pub fn is_table_array(table: &mlua::Table) -> bool {
    if table.is_empty() {
        return table.metatable().is_none();
    };
    for (j, _) in (1..).zip(table.pairs::<mlua::Value, mlua::Value>()) {
        if !table.contains_key(j).unwrap_or(false) {
            return false;
        }
    }
    true
}
