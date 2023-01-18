<p align="center">
  <img width="150px" src="https://raw.githubusercontent.com/github/explore/main/topics/rust/rust.png"/>
</p>

<h3 align="center">
  A simple API the creates and see todos.
</h3>

<h4 align="center">Written in Rust, using Actix, Diesel and GraphQL</h4>

### How to run

First of all, copy the `.env.example` to `.env` file, and update DATABASE_URL var, using your database credentials.

Then, run the project with:

```zsh
cargo run
```

And access `http://localhost:8000/graphiql` in your browser.

Get all todos from  database:

```
query GetTodos {
    todos {
        id
        title
        completed
    }
}
```

Create a new todo:

```
mutation CreateTodo {
  createTodo(newTodo: {
    title: "Update README",
    completed: true,
  }) {
    id,
    title,
    completed
  }
}
```

---

Mady with :purple_heart: by Felipe
