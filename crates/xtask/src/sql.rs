//! Builds a PostgreSQL script of `INSERT`, `UPDATE` and `DELETE` statements as a string.
//!
//! A caller creates a [`Script`], adds statements to it one at a time, and calls
//! [`Script::finalise`] to get the finished SQL as a string:
//!
//! ```ignore
//! let mut script = Script::default();
//! script
//!     .insert_into("ticket_type", Some("ticket_type_id"), &["ticket_type_id", "name"])
//!     .values(&["1, 'Incident'", "2, 'Service Request'"]);
//! script.update("ticket").set(&["status_id = 6"]).where_("ticket_id = 1");
//!
//! let sql = script.finalise();
//! ```
//!
//! The builder trims and lowercases each table name and id column name. It writes every other
//! argument into the SQL exactly as given, including column lists, rows, assignments and
//! conditions. The caller therefore checks those values before passing them in. A misspelt column
//! or an id that matches no row reaches the script unchanged.

use std::fmt::Write;

// ========================================================================================== \\
//                                         Public API                                         \\
// ========================================================================================== \\

/// An ordered list of SQL statements, together with the tables whose identity sequence the script
/// resets at the end.
///
/// An insert that gives an id column writes its ids with `OVERRIDING SYSTEM VALUE`. PostgreSQL
/// leaves the identity sequence where it was when a row supplies its own id, meaning the next
/// insert without an id would reuse one of the fixture's ids. As a result, [`Script::finalise`]
/// ends the script with one `setval` call per such table, moving its sequence to the highest id
/// in the table.
#[derive(Default)]
pub(crate) struct Script {
    statements: Vec<Statement>,
    tables: Vec<(String, String)>,
}

impl Script {
    /// Appends an `INSERT` into `table_name` for `columns` and returns it, ready for
    /// [`Statement::values`].
    ///
    /// Where `id_column` is `Some`, the statement includes `OVERRIDING SYSTEM VALUE`, letting each
    /// row supply its own id. The script also records the table for a `setval` line in
    /// [`Script::finalise`]. It records each table once, keeping the id column from the first
    /// insert into that table.
    ///
    /// `None` suits a table with no identity column, such as one that links two other tables.
    ///
    /// # Examples
    ///
    /// A caller inserts two ticket types with their own ids, then links a ticket to a tag:
    ///
    /// ```ignore
    /// let mut script = Script::default();
    /// script
    ///     .insert_into("ticket_type", Some("ticket_type_id"), &["ticket_type_id", "name"])
    ///     .values(&["1, 'Incident'", "2, 'Service Request'"]);
    /// script
    ///     .insert_into("ticket_tag", None, &["ticket_id", "tag_id"])
    ///     .values(&["1, 2"]);
    ///
    /// let sql = script.finalise();
    /// ```
    ///
    /// The first insert includes `OVERRIDING SYSTEM VALUE` and gains a `setval` line, while the
    /// second gains neither:
    ///
    /// ```text
    /// INSERT
    /// INTO ticket_type (ticket_type_id, name)
    /// OVERRIDING SYSTEM VALUE
    /// VALUES (1, 'Incident'),
    ///        (2, 'Service Request');
    ///
    /// INSERT
    /// INTO ticket_tag (ticket_id, tag_id)
    /// VALUES (1, 2);
    ///
    /// SELECT setval(pg_get_serial_sequence('ticket_type', 'ticket_type_id'), (SELECT max(ticket_type_id) FROM ticket_type));
    /// ```
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

    /// Appends an `UPDATE` of `table_name` and returns it, ready for [`Statement::set`] and
    /// [`Statement::where_`].
    pub(crate) fn update(&mut self, table_name: &str) -> &mut Statement {
        self.add(Statement::update(&normalise(table_name)))
    }

    /// Appends a `DELETE` from `table_name` and returns it, ready for [`Statement::where_`].
    pub(crate) fn delete_from(&mut self, table_name: &str) -> &mut Statement {
        self.add(Statement::delete_from(&normalise(table_name)))
    }

    /// Consumes the script and returns its SQL.
    ///
    /// The SQL lists the statements in the order the caller added them, each ending in `;` and
    /// followed by a blank line. One `setval` line per recorded table comes thereafter, in the
    /// order the tables were first inserted into:
    ///
    /// ```text
    /// INSERT
    /// INTO ticket_type (ticket_type_id, name)
    /// OVERRIDING SYSTEM VALUE
    /// VALUES (1, 'Incident'),
    ///        (2, 'Service Request');
    ///
    /// SELECT setval(pg_get_serial_sequence('ticket_type', 'ticket_type_id'), (SELECT max(ticket_type_id) FROM ticket_type));
    /// ```
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

    /// Appends `statement` to the script and returns a mutable reference to it in its new place.
    fn add(&mut self, statement: Statement) -> &mut Statement {
        self.statements.push(statement);
        self.statements
            .last_mut()
            .expect("the statement pushed on the line above is the last one")
    }

    /// Records `table_name` and its `id_column` for a `setval` line, unless the script has
    /// recorded that table already.
    fn track(&mut self, table_name: &str, id_column: &str) {
        let tracked = self.tables.iter().any(|(table, _)| table == table_name);
        if !tracked {
            self.tables
                .push((table_name.to_owned(), id_column.to_owned()));
        }
    }
}

/// One SQL statement, built up one clause at a time.
///
/// Each clause method appends its clause to the end of the statement and returns the same
/// statement. A caller therefore chains the clauses in the order SQL expects them. The statement
/// text ends without a `;`, which [`Script::finalise`] adds.
pub(crate) struct Statement(String);

impl Statement {
    // Constructors:
    /// Starts an `INSERT` into `table_name` for `columns`, adding `OVERRIDING SYSTEM VALUE` when
    /// `overriding` is `true`.
    fn insert_into(table_name: &str, overriding: bool, columns: &[&str]) -> Self {
        let mut stmt = format!("INSERT\nINTO {table_name} ({})", columns.join(", "));
        if overriding {
            stmt.push_str("\nOVERRIDING SYSTEM VALUE");
        }

        Self(stmt)
    }

    /// Starts an `UPDATE` of `table_name`.
    fn update(table_name: &str) -> Self {
        Self(format!("UPDATE {table_name}"))
    }

    /// Starts a `DELETE` from `table_name`.
    fn delete_from(table_name: &str) -> Self {
        Self(format!("DELETE\nFROM {table_name}"))
    }

    /// Appends a `VALUES` clause with one row for each entry in `rows`.
    ///
    /// Each entry is the comma-separated SQL literals of one row, without brackets. The method
    /// wraps each entry in brackets and puts it on its own line.
    ///
    /// # Examples
    ///
    /// A caller passes one string per row:
    ///
    /// ```ignore
    /// let mut script = Script::default();
    /// script
    ///     .insert_into("ticket_status", Some("status_id"), &["status_id", "name"])
    ///     .values(&["1, 'New'", "2, 'In Progress'"]);
    /// ```
    ///
    /// The statement then ends in this clause:
    ///
    /// ```text
    /// VALUES (1, 'New'),
    ///        (2, 'In Progress')
    /// ```
    pub(crate) fn values(&mut self, rows: &[&str]) -> &mut Self {
        let rows: Vec<String> = rows.iter().map(|row| format!("({row})")).collect();
        self.push("\nVALUES ");
        self.push(&rows.join(",\n       "));

        self
    }

    /// Appends a `SET` clause that joins `assignments` with commas.
    ///
    /// Each entry is one complete assignment, such as `"status_id = 6"`.
    ///
    /// # Examples
    ///
    /// A caller moves a ticket to status 6 and sets its priority to 3 in one statement:
    ///
    /// ```ignore
    /// let mut script = Script::default();
    /// script
    ///     .update("ticket")
    ///     .set(&["status_id = 6", "priority = 3"])
    ///     .where_("ticket_id = 1");
    /// ```
    ///
    /// The script then contains this statement:
    ///
    /// ```text
    /// UPDATE ticket
    /// SET status_id = 6, priority = 3
    /// WHERE ticket_id = 1
    /// ```
    pub(crate) fn set(&mut self, assignments: &[&str]) -> &mut Self {
        self.push("\nSET ");
        self.push(&assignments.join(", "));

        self
    }

    /// Appends a `WHERE` clause containing `condition`, such as `"ticket_id = 1"`.
    ///
    /// The trailing underscore is there because `where` is a Rust keyword.
    ///
    /// # Examples
    ///
    /// A caller deletes one ticket by its id:
    ///
    /// ```ignore
    /// let mut script = Script::default();
    /// script.delete_from("ticket").where_("ticket_id = 1");
    /// ```
    ///
    /// The script then contains this statement:
    ///
    /// ```text
    /// DELETE
    /// FROM ticket
    /// WHERE ticket_id = 1
    /// ```
    pub(crate) fn where_(&mut self, condition: &str) -> &mut Self {
        self.push("\nWHERE ");
        self.push(condition);

        self
    }

    /// Appends `sql` to the statement text.
    fn push(&mut self, sql: &str) {
        self.0.push_str(sql);
    }
}

// ========================================================================================== \\
//                                          Helpers                                           \\
// ========================================================================================== \\

/// Trims and lowercases `name`, meaning `" TICKET_TYPE "` and `"ticket_type"` identify the same
/// table. PostgreSQL lowercases an unquoted identifier in the same way.
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
