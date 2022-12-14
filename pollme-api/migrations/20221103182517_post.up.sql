-- Add up migration script here
CREATE TABLE IF NOT EXISTS public.post (
  id serial NOT NULL,
  title character varying NOT NULL,
  created_at timestamp without time zone NOT NULL DEFAULT NOW(),
  user_id int NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (user_id) REFERENCES "user"(id)
);