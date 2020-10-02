# subject-ms

This microservice handles the details and contents of a Subject. The relation of Subjects with Teachers can be handled as well.

Built with [Rocket](rocket.rs) and [Diesel](diesel.rs).

You must provide a (MySQL) database URL with credentials in `Rocket.toml` to configure Rocket's Connection Pool.
Migrations are also provided in order to set up the database.

Juan Camilo Vargas [/jcvar](https://github.com/jcvar)

Facultad de Ingeniería\
Universidad Nacional de Colombia\
2020-2
