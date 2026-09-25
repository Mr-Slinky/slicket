---
status: accepted
date: 2026-09-25
decision-makers: Kheagen Haskins
---

# Render the frontend on the server with askama and htmx

## Context and Problem Statement

Slicket needs a frontend in the browser, where the people in each organisation log and follow their
tickets. The `slicket-server` crate is the only part of Slicket that a browser can reach, so the
frontend talks to it over HTTP.

There are two broad ways to build that frontend. In the first, the server exposes a JSON API, and
JavaScript in the browser builds the page from the data it receives. In the second, the server builds
the HTML itself and sends finished pages, or pieces of pages, to the browser.

The work in Slicket is mostly lists, forms and detail pages: a list of tickets, a ticket with its
history, a form that logs a new one. Each of those is a request followed by a page that shows the
result.

## Decision Drivers

* A small frontend. The developer judged that "react is more than likely far too heavy for what we
  want here".
* Hand-rolled code. In the developer's words, "hand rolling is somewhat the point (within reason of
  course)".
* One copy of the state. The server decides what a ticket looks like, so the browser keeps no second
  copy that could fall out of step with it.
* More of the work in Rust, since staying current with Rust is one of the project's goals.
* Components that the frontend reuses across pages, such as a ticket row or a status badge.

## Considered Options

* Server-rendered HTML with askama, updated in place by htmx
* A JSON API with hand-written Web Components
* A JSON API with Lit
* A JSON API with React

## Decision Outcome

Chosen option: "Server-rendered HTML with askama, updated in place by htmx", because it keeps the
state and nearly all the logic on the server, in Rust, while the browser runs one small script.

The browser loads htmx, a JavaScript library, and the pages use its attributes on ordinary HTML
elements. A button marked `hx-post="/tickets/42/close"` sends that request when clicked. The server
closes the ticket and renders the updated ticket row as HTML. htmx then swaps that row into the page
in place of the old one.

The server renders every page and fragment with askama. An askama template is an HTML file with
placeholders, and a Rust struct supplies the values that fill them. askama compiles each template
into the binary at build time. As such, a template that refers to a field the struct lacks fails to
compile.

A reusable component is an askama template that other templates include, such as a ticket row that
both the ticket list and the ticket page render.

### Consequences

* Good, because the server holds the only copy of the state, so the page always shows what the
  server last decided.
* Good, because the logic lives in Rust, and the browser runs htmx with no build step.
* Good, because a mistake in a template is a compile error.
* Good, because each HTML fragment serves exactly the page that uses it, so no endpoint has to suit
  several clients.
* Bad, because every interaction makes a round trip to the server. A highly interactive widget, such
  as a drag-and-drop board, suits this poorly. A single Web Component in that page can handle it.
* Bad, because Slicket exposes no API for other programs. An integration, such as email-to-ticket or
  a mobile app, needs JSON endpoints added next to the HTML ones.
* Bad, because a change to a template means recompiling the server, since askama builds templates
  into the binary.

## Pros and Cons of the Options

### Server-rendered HTML with askama, updated in place by htmx

The server renders HTML from templates, and htmx swaps the returned fragments into the page.

* Good, because the state and the logic stay on the server, in Rust.
* Good, because askama checks every template at compile time.
* Good, because the browser needs one script and no build step.
* Bad, because each interaction waits on the network.
* Bad, because other programs need a separate JSON API.

### A JSON API with hand-written Web Components

The server returns JSON, and the browser builds the page from custom elements that extend
`HTMLElement`, with no library.

* Good, because the browser provides the whole component model, so the frontend has no dependencies.
* Good, because the JSON API also serves other programs.
* Bad, because each component updates the DOM by hand whenever its data changes.
* Bad, because the browser keeps its own copy of the state, which the code has to keep in step with
  the server.
* Bad, because much of the logic moves out of Rust and into JavaScript.

### A JSON API with Lit

The same as hand-written Web Components, with Lit adding reactive properties and templates on top.

* Good, because Lit re-renders a component when its data changes.
* Good, because the JSON API also serves other programs.
* Bad, because the browser keeps its own copy of the state.
* Bad, because much of the logic moves out of Rust and into JavaScript.

### A JSON API with React

The server returns JSON, and a React application builds the page in the browser.

* Good, because React has the largest set of ready-made components and tools.
* Good, because the JSON API also serves other programs.
* Bad, because it brings a build toolchain and a large dependency tree, which the developer judged
  too heavy for Slicket.
* Bad, because the browser keeps its own copy of the state.
* Bad, because little of the frontend is hand-rolled.
