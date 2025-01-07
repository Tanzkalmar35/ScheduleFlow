# General
As for the database, I chose to go with a self-hosted psql db.

To connect this app to your self-hosted database, you need to do the following.

1. Create a .env file:

You need a .env file that includes the accessors to your db. The .env file needs to include:
- PSQL_NAME: The name of the db
- PSQL_USER: The username for the db
- PSQL_PASS: The password for the db
- PSQL_IP: The ip address that points to your db
- SCHEDULEFLOW_JWT_SECRET: The secret the jwt sessions are generated with

Make sure to not put that .env file anywhere except your pc, so the credentials to your db don't get exposed!

2. Create the db itself

As for the tables, you can use this schema:

```sql
CREATE TABLE public.calendars (
                                  "uuid" uuid NOT NULL,
                                  "name" varchar NULL,
                                  CONSTRAINT calendars_pkey PRIMARY KEY (uuid)
);

CREATE TABLE public.properties (
                                   "uuid" uuid NOT NULL,
                                   "key" varchar NOT NULL,
                                   value varchar NOT NULL,
                                   owner_uuid uuid NOT NULL,
                                   owner_type varchar NOT NULL,
                                   CONSTRAINT properties_pkey PRIMARY KEY (uuid)
);

CREATE TABLE public.users (
                              "uuid" uuid NOT NULL,
                              email varchar(255) NULL,
                              "password" varchar(255) NULL,
                              username varchar(255) NULL,
                              CONSTRAINT users_pkey PRIMARY KEY (uuid)
);

CREATE TABLE public.components (
                                   "uuid" uuid NOT NULL,
                                   c_type varchar(255) NULL,
                                   calendar_uuid uuid NULL,
                                   CONSTRAINT components_pkey PRIMARY KEY (uuid),
                                   CONSTRAINT components_calendar_uuid_fkey FOREIGN KEY (calendar_uuid) REFERENCES public.calendars("uuid")
);

CREATE TABLE public.user_jwt_tokens (
                                        "token" varchar NOT NULL,
                                        user_uuid uuid NOT NULL,
                                        CONSTRAINT user_jwt_tokens_pkey PRIMARY KEY (token),
                                        CONSTRAINT user_uuid_fkey FOREIGN KEY (user_uuid) REFERENCES public.users("uuid")
);

CREATE TABLE public.users_calendars (
                                        calendar_uuid uuid NOT NULL,
                                        user_uuid uuid NOT NULL,
                                        CONSTRAINT calendar_users_pkey PRIMARY KEY (calendar_uuid, user_uuid),
                                        CONSTRAINT calendar_users_calendar_uuid_fkey FOREIGN KEY (calendar_uuid) REFERENCES public.calendars("uuid"),
                                        CONSTRAINT calendar_users_user_uuid_fkey FOREIGN KEY (user_uuid) REFERENCES public.users("uuid")
);

CREATE TABLE public.users_clients (
	user_uuid uuid NOT NULL,
	client_uuid uuid NOT NULL,
	CONSTRAINT users_clients_pkey PRIMARY KEY (client_uuid, user_uuid),
	CONSTRAINT users_clients_clients_fk FOREIGN KEY (client_uuid) REFERENCES public.clients("uuid"),
	CONSTRAINT users_clients_users_fk FOREIGN KEY (user_uuid) REFERENCES public.users("uuid")
);

CREATE TABLE public.clients (
	"uuid" uuid NOT NULL,
	"name" varchar NOT NULL,
	CONSTRAINT clients_pk PRIMARY KEY (uuid)
);
```
