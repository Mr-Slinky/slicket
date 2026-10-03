//! Builds the SQL statements that the fixture generator writes to a fixture file.
//! Does not do validation on its inputs, and so can produce non-sensical SQL or incorrect references.
//! Calling modules are responsible for validating the values before inputting them herein.

use std::fmt::Write;

// Notes: (AI to replace these notes with real documentation when the user requests)
// Script must track all tables
#[derive(Default)]
pub(crate) struct Script {
    script: String,
    tables: Vec<(String, String)>,
}

impl Script {
    pub(crate) fn insert_into(
        &mut self,
        table_name: &str,
        id_column: Option<&str>,
        columns: &[&str],
    ) -> Statement {
        let table_name = normalise(table_name);
        if let Some(id_column) = id_column {
            self.track(&table_name, &normalise(id_column));
        }

        Statement::insert_into(&table_name, id_column.is_some(), columns)
    }

    pub(crate) fn update(&self, table_name: &str) -> Statement {
        Statement::update(&normalise(table_name))
    }

    pub(crate) fn delete_from(&self, table_name: &str) -> Statement {
        Statement::delete_from(&normalise(table_name))
    }

    pub(crate) fn push(&mut self, statement: Statement) {
        self.script.push_str(&statement.0);
        self.script.push_str(";\n\n");
    }

    pub(crate) fn finalise(mut self) -> String {
        for (table_name, id_column) in &self.tables {
            writeln!(
                self.script,
                "SELECT setval(pg_get_serial_sequence('{table_name}', '{id_column}'), (SELECT max({id_column}) FROM {table_name}));"
            )
            .unwrap();
        }

        self.script
    }

    fn track(&mut self, table_name: &str, id_column: &str) {
        let tracked = self.tables.iter().any(|(table, _)| table == table_name);
        if !tracked {
            self.tables
                .push((table_name.to_owned(), id_column.to_owned()));
        }
    }
}

#[must_use = "a Statement does nothing until it is pushed onto a Script"]
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

    pub(crate) fn values(mut self, rows: &[&str]) -> Self {
        let rows: Vec<String> = rows.iter().map(|row| format!("({row})")).collect();
        self.push("\nVALUES ");
        self.push(&rows.join(",\n       "));

        self
    }

    pub(crate) fn set(mut self, assignments: &[&str]) -> Self {
        self.push("\nSET ");
        self.push(&assignments.join(", "));

        self
    }

    pub(crate) fn where_(mut self, condition: &str) -> Self {
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

        let stmt = script
            .insert_into(
                "ticket_type",
                Some("ticket_type_id"),
                &["ticket_type_id", "name"],
            )
            .values(&["1, 'Incident'", "2, 'Service Request'"]);
        script.push(stmt);
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

        let stmt = script
            .insert_into("ticket_tag", None, &["ticket_id", "tag_id"])
            .values(&["1, 2"]);
        script.push(stmt);
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

        let first = script
            .insert_into("ticket_type", Some("ticket_type_id"), &["ticket_type_id", "name"])
            .values(&["1, 'Incident'"]);
        script.push(first);
        let second = script
            .insert_into(" TICKET_TYPE ", Some("Ticket_Type_Id"), &["ticket_type_id", "name"])
            .values(&["2, 'Service Request'"]);
        script.push(second);
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

        let stmt = script
            .update("ticket")
            .set(&["status = 'Closed'", "priority = 3"])
            .where_("ticket_id = 1");
        script.push(stmt);
        let actual = script.finalise();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_delete_from_with_valid_args_returns_expected_sql() {
        let mut script = Script::default();
        let expected = concat!(
            "DELETE\n",
            "FROM ticket\n",
            "WHERE ticket_id = 1;\n",
            "\n",
        );

        let stmt = script.delete_from("ticket").where_("ticket_id = 1");
        script.push(stmt);
        let actual = script.finalise();

        assert_eq!(expected, actual);
    }
}
