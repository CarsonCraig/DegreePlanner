# UW CoursePlan

The backend API for UW CoursePlan.

## Development Setup

1. Install [rustc and cargo] (the Rust compiler and build tool)
2. Install [PostgreSQL] 10.5 or above (step 3 and 4 may happen as part of the installer)
3. Setup PostgreSQL user:
   ```
   $ sudo -u postgres psql
   \password postgres
   # Enter a password (Don't use a valuable password for this)
   \q
   ```
4. Install [pgAdmin] 4 (helpful GUI for PostgreSQL)
5. Run pgAdmin (a window should open with the pgAdmin interface)
6. Click "Add New Server" (You need to connect to your local PostgreSQL instance)
    * **General** > **Name**: localhost
    * **Connection** > **Host**: localhost
    * **Connection** > **Username**: postgres
    * **Connection** > **Password**: (The password you setup earlier)
    * **Connection** > **Save password?**: Yes
7. Install [Diesel] with `cargo install diesel_cli --no-default-features --features postgres`
8. (Recommended) Setup [bash completion for Diesel]
9. Clone the repository if you haven't already
10. Run `cp .env.template .env`
11. Edit the `.env` file to contain the configuration for your database
    * Make sure to replace `username` with `postgres` and `password` with your password
12. Run `diesel setup` (this will automatically create and setup a database for you in PostgreSQL)
13. Edit your `/etc/hosts` file (`C:\Windows\System32\Drivers\etc\hosts` on
    Windows) and add the following lines:
    ```hosts
    # For UW CoursePlan project
    127.0.0.1	local.uwcourseplan.com
    ```

[rustc and cargo]: https://rustup.rs/
[PostgreSQL]: https://www.postgresql.org/
[pgAdmin]: https://www.pgadmin.org/
[Diesel]: http://diesel.rs/guides/getting-started/
[bash completion for Diesel]: https://github.com/diesel-rs/diesel/tree/master/diesel_cli#bash-completion

## Building & Running

Once you have everything setup, you can run the following to launch the server:

```
cargo run
```

To update to the latest database migrations, you can run:

```
diesel migration run
```
