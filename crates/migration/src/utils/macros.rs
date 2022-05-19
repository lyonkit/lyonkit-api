macro_rules! exec_stmt {
  ($manager: ident, $($arg:tt)+) => {{
    use crate::sea_orm::ConnectionTrait;
    $manager
      .get_connection()
      .execute(
        crate::sea_orm::Statement::from_string(
          $manager.get_database_backend(),
          format!("{}", format_args!($($arg)+))
        )
      )
      .await
      .map(|_| ())
  }};
}

macro_rules! create_table_from_entity {
  ($manager: ident, $entity: ident) => {{
    $manager
      .create_table(
        sea_orm::Schema::new($manager.get_database_backend()).create_table_from_entity($entity),
      )
      .await
  }};
}

pub(crate) use create_table_from_entity;
pub(crate) use exec_stmt;
