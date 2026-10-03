//! Builds the SQL statements that the fixture generator writes to a fixture file.
//! Does not do validation on its inputs, and so can produce non-sensical SQL or incorrect references.
//! Calling modules are responsible for validating the values before inputting them herein.

use std::fmt::Write;

// Notes: (AI to replace these notes with real documentation when the user requests)
// Script must track all tables
#[derive(Default)]
pub(crate) struct Script {
    statements: Vec<Statement>,
    tables: Vec<(String, String)>,
}

impl Script {
    pub(crate) fn insert_into(
        &mut self,
        table_name: &str,
        id_column: Option<&str>,
        columns: &[&str],
    ) -> &mut Statement {
        let table_name = normalise(table_name);
        if let Some(id_column) = id_column {
            self.track(&table_name, &normalise(id_column));
        }

        self.add(Statement::insert_into(
            &table_name,
            id_column.is_some(),
            columns,
        ))
    }

    pub(crate) fn update(&mut self, table_name: &str) -> &mut Statement {
        self.add(Statement::update(&normalise(table_name)))
    }

    pub(crate) fn delete_from(&mut self, table_name: &str) -> &mut Statement {
        self.add(Statement::delete_from(&normalise(table_name)))
    }

    pub(crate) fn finalise(self) -> String {
        let mut script = String::new();
        for statement in &self.statements {
            script.push_str(&statement.0);
            script.push_str(";\n\n");
        }

        for (table_name, id_column) in &self.tables {
            writeln!(
                script,
                "SELECT setval(pg_get_serial_sequence('{table_name}', '{id_column}'), (SELECT max({id_column}) FROM {table_name}));"
            )
            .unwrap();
        }

        script
    }

    fn add(&mut self, statement: Statement) -> &mut Statement {
        self.statements.push(statement);
        self.statements
            .last_mut()
            .expect("the statement pushed on the line above is the last one")
    }

    fn track(&mut self, table_name: &str, id_column: &str) {
        let tracked = self.tables.iter().any(|(table, _)| table == table_name);
        if !tracked {
            self.tables
                .push((table_name.to_owned(), id_column.to_owned()));
        }
    }
}

pub(crate) struct Statement(String);

impl Statement {
    // Constructors:
    fn insert_into(table_name: &str, overriding: bool, columns: &[&str]) -> Self {
        let mut stmt = format!("INSERT\nINTO {table_name} ({})", columns.join(", "));
        if overriding {
            stmt.push_str("\nOVERRIDING SYSTEM VALUE");
        }

        Self(stmt)
    }

    fn update(table_name: &str) -> Self {
        Self(format!("UPDATE {table_name}"))
    }

    fn delete_from(table_name: &str) -> Self {
        Self(format!("DELETE\nFROM {table_name}"))
    }

    pub(crate) fn values(&mut self, rows: &[&str]) -> &mut Self {
        let rows: Vec<String> = rows.iter().map(|row| format!("({row})")).collect();
        self.push("\nVALUES ");
        self.push(&rows.join(",\n       "));

        self
    }

    pub(crate) fn set(&mut self, assignments: &[&str]) -> &mut Self {
        self.push("\nSET ");
        self.push(&assignments.join(", "));

        self
    }

    pub(crate) fn where_(&mut self, condition: &str) -> &mut Self {
        self.push("\nWHERE ");
        self.push(condition);

        self
    }

    fn push(&mut self, sql: &str) {
        self.0.push_str(sql);
    }
}

fn normalise(name: &str) -> String {
    name.trim().to_lowercase()
}

// ========================================================================================== \\
//                                           Tests                                            \\
// ========================================================================================== \\

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_into_with_valid_args_returns_expected_sql() {
        let mut script = Script::default();
        let expected = concat!(
            "INSERT\n",
            "INTO ticket_type (ticket_type_id, name)\n",
            "OVERRIDING SYSTEM VALUE\n",
            "VALUES (1, 'Incident'),\n",
            "       (2, 'Service Request');\n",
            "\n",
            "SELECT setval(pg_get_serial_sequence('ticket_type', 'ticket_type_id'), (SELECT max(ticket_type_id) FROM ticket_type));\n",
        );

        script
            .insert_into(
                "ticket_type",
                Some("ticket_type_id"),
                &["ticket_type_id", "name"],
            )
            .values(&["1, 'Incident'", "2, 'Service Request'"]);
        let actual = script.finalise();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_insert_into_with_no_id_column_returns_sql_without_setval() {
        let mut script = Script::default();
        let expected = concat!(
            "INSERT\n",
            "INTO ticket_tag (ticket_id, tag_id)\n",
            "VALUES (1, 2);\n",
            "\n",
        );

        script
            .insert_into("ticket_tag", None, &["ticket_id", "tag_id"])
            .values(&["1, 2"]);
        let actual = script.finalise();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_insert_into_with_same_table_twice_returns_one_setval() {
        let mut script = Script::default();
        let expected = concat!(
            "INSERT\n",
            "INTO ticket_type (ticket_type_id, name)\n",
            "OVERRIDING SYSTEM VALUE\n",
            "VALUES (1, 'Incident');\n",
            "\n",
            "INSERT\n",
            "INTO ticket_type (ticket_type_id, name)\n",
            "OVERRIDING SYSTEM VALUE\n",
            "VALUES (2, 'Service Request');\n",
            "\n",
            "SELECT setval(pg_get_serial_sequence('ticket_type', 'ticket_type_id'), (SELECT max(ticket_type_id) FROM ticket_type));\n",
        );

        script
            .insert_into(
                "ticket_type",
                Some("ticket_type_id"),
                &["ticket_type_id", "name"],
            )
            .values(&["1, 'Incident'"]);
        script
            .insert_into(
                " TICKET_TYPE ",
                Some("Ticket_Type_Id"),
                &["ticket_type_id", "name"],
            )
            .values(&["2, 'Service Request'"]);
        let actual = script.finalise();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_update_with_valid_args_returns_expected_sql() {
        let mut script = Script::default();
        let expected = concat!(
            "UPDATE ticket\n",
            "SET status = 'Closed', priority = 3\n",
            "WHERE ticket_id = 1;\n",
            "\n",
        );

        script
            .update("ticket")
            .set(&["status = 'Closed'", "priority = 3"])
            .where_("ticket_id = 1");
        let actual = script.finalise();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_delete_from_with_valid_args_returns_expected_sql() {
        let mut script = Script::default();
        let expected = concat!("DELETE\n", "FROM ticket\n", "WHERE ticket_id = 1;\n", "\n",);

        script.delete_from("ticket").where_("ticket_id = 1");
        let actual = script.finalise();

        assert_eq!(expected, actual);
    }
}
