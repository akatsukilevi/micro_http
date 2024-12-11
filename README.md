# Micro HTTP Server

This is a Micro HTTP Server template, meant to be the smallest real-use webserver, with only around `650` tree-dependencies:
- `tiny_http` - Tiny HTTP server, handles all the HTTP stuff
- `matchit` - A small router, meant for separating routes into functions
- `clap` - CLI handling, bit bulky but needed to handle everything cli-related
- `sea-orm` - Database handler, could be swapped for SQLx or anything else
- `tera` - Templating for the webpages
- `htmx` - Interactivity with server-side, improves the vanilla HTML experience
- `alpinejs` - Client-side interactivity
- `picocss` - CSS Styling to make things look better
- `leaflet` - For when you need a simplistic map
- `vite` - Bundling htmx, alpinejs, picocss and leaflet so that a external CDN is not needed

## Dependencies

In terms of dependencies, not much is needed, only really needing Rust and Node.JS to be installed.

With some small tweaks, such as removing `vite`, the `package.json` files, and replacing it's links on the `views/templates/base.html` template file, and removing the `build.rs`, can completely remove the dependency on Node.JS, at the cost of needing a external CDN for things like CSS and JS libraries you might want to use.

## Explaining the structure

The actual code structure is meant to be something simplistic, while also allowing to quickly expand the codebase for new features:

- `src/cli` - Everything related to the command-line. Can be expanded to add new commands
- `src/config` - General-purpose configuration for the whole application
- `src/core` - Core functionality such as error handling and routing
- `src/database/migrations` - Embedded migrations for the database
- `src/database/models` - The database binding models
- `src/providers` - Any sort of external dependency such as a external KV, a Database, Templating provider, etc
- `src/services` - The actual routes served
- `src/web_server` - The entrypoint for the web server, initializes the providers and spawns in the http server.
- `views` - The page templates that are rendered in
- `views/templates` - The skeleton templates that the pages uses for things like layout or reusable components

## The Why's

### Why the `main.js` at the `views` folder?

Vite. Literally, Vite. For the bundling to work, we need to set a entrypoint that not only imports the dependencies you need, but also sets them in the proper place.

### How do I add X library?

At the moment, such is a two-fold work.
You first must edit the `views/main.js` file to import it, then add it to the `window` context as needed (then also do any initialization step you might need).
Then, you must add it to a chunk at the `manualChunks` section inside the `vite.config.ts`.

In future, I might find a way to condense this into a simpler method, but for now it's the best I can do :3

### How do I use Vue/React/Angular?

This, is not the project for this. You probably can do it, yet don't expect much support with it. The goal of this project is to have a minimal code-base, while leveraging Vanilla HTML/CSS/JS.

The incrementation with HTMX and Alpine.JS are meant to facilitate small things such as forms, small interactivities and others.

### Why not `SQLx`/`Diesel`/`[Insert some other SQL library here]`?

Without going into all the model-relationship system (which, for me, is quite redundant), SeaORM feels more akin to a query builder than a ORM itself.
Yet, this is not forced, you can simply replace it with whatever other library you'd like to handle database, edit the `src/providers/database.rs` file to use your library of choice instead, and go your merry way

### Why not `MongoDB`?

No. Just no.

## Something broke!

Try to nuke the `target` folder and recompile from scratch. Sometimes, it does that

## There is no hot-reloading for the templates!

I couldn't figure out a decent way to have hot-reloading working on this. Feel free to give it a try if you'd like, do send a PR if you can.

I'm aware of Tera's hot reloading module(`tera-hot-reload`), but for some god-forsaken reason, it depends on Tower for compatibility with `axum` and others, while not having it behind a feature-flag. Not wanting to have the entirety of `axum` and `tower` on my dependency tree when I'm not even using them.

## License

This project is mostly meant as a 'do whatever you want, just give credits', but since the world isn't as bright as one would hope...

  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at https://mozilla.org/MPL/2.0/.
