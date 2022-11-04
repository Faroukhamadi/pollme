-- Add up migration script here
CREATE TABLE IF NOT EXISTS public.user (
  id serial NOT NULL,
  username character varying NOT NULL,
  password character varying NOT NULL,
  created_at timestamp without time zone NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id),
  CONSTRAINT username_unique UNIQUE (username)
);