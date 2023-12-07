macro_rules! entities {
    ($schema:ident,$manager:ident, $($entity_type:path),*) => {
        $(
        {
            let statement = $schema.create_table_from_entity($entity_type);
            $manager.create_table(statement).await?;
        }
        )*
    };
}
pub(crate) use entities;
