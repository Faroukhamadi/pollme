-- Add up migration script here
CREATE TABLE IF NOT EXISTS public.choice (
  id serial NOT NULL,
  name character varying NOT NULL,
  created_at timestamp without time zone NOT NULL DEFAULT NOW(),
  user_id integer NOT NULL,
  post_id integer NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (user_id) REFERENCES "user"(id),
  FOREIGN KEY (post_id) REFERENCES post(id),
  CONSTRAINT choice_user_post_unique UNIQUE (user_id, post_id)
);