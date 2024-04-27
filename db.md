# General
As for the database, I chose to go with a self-hosted psql db.

To connect this app to your self-hosted database, you need to do the following.

1. Create a .env file:

You need a .env file that includes the accessors to your db. The .env file needs to include:
- PSQL_NAME: The name of the db
- PSQL_USER: The username for the db
- PSQL_PASS: The password for the db
- PSQL_IP: The ip address that points to your db

Make sure to not put that .env file anywhere except your pc, so the credentials to your db don't get exposed!

2. Create the db itself

As for the tables, you can use this schema:

```sql
CREATE TABLE users (
                       uuid uuid PRIMARY key unique,
                       email VARCHAR(255),
                       password VARCHAR(255),
                       username VARCHAR(255)
);

CREATE TABLE calendars (
                           uuid uuid PRIMARY key unique
);

CREATE TABLE properties (
                            uuid uuid PRIMARY key unique,
                            key VARCHAR(255),
                            value VARCHAR(255)
);

CREATE TABLE calendars_properties (
                                      calendar_id uuid,
                                      property_id uuid,
                                      PRIMARY KEY (calendar_id, property_id),
                                      FOREIGN KEY (calendar_id) REFERENCES calendars (uuid),
                                      FOREIGN KEY (property_id) REFERENCES properties (uuid)
);

CREATE TABLE components (
                            uuid uuid PRIMARY key unique,
                            type VARCHAR(255)
);

CREATE TABLE calendars_components (
                                      calendar_id uuid,
                                      component_id uuid,
                                      PRIMARY KEY (calendar_id, component_id),
                                      FOREIGN KEY (calendar_id) REFERENCES calendars (uuid),
                                      FOREIGN KEY (component_id) REFERENCES components (uuid)
);
```
