# CLAUDE.md

This file guides AI agents working in the Slicket repository. The global CLAUDE.md still applies,
apart from its Java, Maven and Log4j2 sections. Cargo takes the place of Maven here.

## Project

Slicket is a ticketing system. A tenant is the business that runs an instance of Slicket. The
tenant sets up the organisations it serves and the people in each one. Those people then log
tickets.

An organisation is a customer of the tenant, and each person belongs to one organisation. An
organisation owns nothing else in the schema.

The inspiration is a well-known ITSM and PSA platform, which the README calls the holy ticketing
system. The developer has used it for a long time and finds it complicated and unintuitive, so
Slicket aims to feel far better to use. The aim is a better experience rather than feature parity
with the holy ticketing system. Agents refer to that platform by this nickname alone, in every file
and every commit message.

The developer has three goals for the project:

- Build the improvements they spot in the software they use every day, to prove that they can.
- Stay current with Rust over a project that runs for years.
- Model the backend as an Entity Component System (ECS), as a deliberate challenge and for learning.

## Workspace

Slicket is a Cargo workspace. Each crate lives in its own folder under `crates/`. The root
`Cargo.toml` lists every crate in `members`. The commands below run from the repository root.

| Task              | Command                             |
|-------------------|-------------------------------------|
| Build every crate | `cargo build`                       |
| Test every crate  | `cargo test`                        |
| Build one crate   | `cargo build -p <crate>`            |
| Lint every crate  | `cargo clippy --workspace`          |
| Add a dependency  | `cargo add -p <crate> <dependency>` |

## Working with the developer

The developer wants to understand and own every line of Slicket. The developer writes the code, and
AI agents help with that understanding and act as peer reviewers.

### Discussion and instructions

A message that discusses an idea, a design or a problem gets a reply in conversation, and the agent
builds nothing. Building starts only when a message asks for it in plain words, such as "write",
"implement", "fix" or "commit". Where a message could be either, the agent treats it as discussion.

| Message                                                | Treat as    |
|--------------------------------------------------------|-------------|
| "I'm wondering whether tickets should be entities."    | Discussion  |
| "How would you model an employee's permissions?"       | Discussion  |
| "Write a function that closes a ticket."               | Instruction |

### Areas open to agents

Agents have free rein in four areas:

1. **Documentation and comments.** Agents write and edit doc comments, inline comments, READMEs and
   anything under `docs/`.
2. **Rust concepts.** Agents explain any Rust concept the developer is still getting to grips with.
   The developer knows Java well, including Maven multi-module builds and JPMS. A comparison with
   the Java counterpart often helps. Where the agent draws one, it also says where the comparison
   stops holding true.
3. **One function at a time.** An agent writes a single, clearly defined function or method per
   request. The request states what the function takes, what it returns and what it does. Where any
   of those is unclear, the agent asks first. After writing the function, the agent explains the
   Rust concepts it relies on, then stops.
4. **Git.** Agents write every commit message, the developer's included. An agent commits its own
   changes once the developer has reviewed them, as the Commits section sets out. It commits the developer's work, pushes
   and pulls each time the developer asks.

Every other change waits for an explicit instruction from the developer. That includes writing more
than one function, adding types, modules or crates, restructuring code, adding dependencies and
changing the logic of code the developer wrote.

### Code review

Most requests ask for a review of code the developer wrote. The review exists to improve the
developer's Rust.

- The agent orders findings by importance: bugs first, then non-idiomatic Rust, then style.
- Each finding states the problem and the Rust reason behind it, such as ownership, borrowing,
  lifetimes or a standard-library idiom.
- A finding can include a short snippet in chat that shows the fix. The developer then makes the
  change in the code.
- The agent can flag an item marked `pub` that no code outside its crate needs yet, such as the
  inner value of an id type. `pub(crate)` is the default until another crate needs the item. This
  is a decision based on the cost of changing visibility later. Widening `pub(crate)` to `pub`
  breaks no code, while narrowing `pub` to `pub(crate)` breaks every other crate that uses the item.

## Commits

Commits follow the Conventional Commits format from the `technical-writing` skill.

The developer's work and an agent's work go in separate commits. The git author field on each
commit records whose work it contains:

| Work            | Author                                 |
|-----------------|----------------------------------------|
| The developer's | The developer, as `git config` sets it |
| An agent's      | `Claude <noreply@anthropic.com>`       |

An agent commits its own changes only after the developer has reviewed them and says to commit.
Until then, the agent reports that the change is ready for review and stops. At commit time, the
agent stages only the files it changed, each by name, so each commit contains one person's work.
It also sets the author explicitly:

```
git commit --author="Claude <noreply@anthropic.com>" -m "docs: explain the Entity generation"
```

Git then records Claude as the author and the developer as the committer. `git log --author=Claude`
lists every agent commit.

A review suggestion that the developer applies in the code is the developer's work, so it goes in a
developer commit.

For this repository, these rules replace the global CLAUDE.md rule against AI attribution in
commits. The message itself credits no one, and the `Co-Authored-By` trailer stays out.
