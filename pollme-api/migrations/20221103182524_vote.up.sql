-- Add up migration script here
CREATE TABLE IF NOT EXISTS public.vote (
  id serial NOT NULL,
  inc bigint NOT NULL,
  created_at timestamp without time zone NOT NULL DEFAULT NOW(),
  user_id bigint NOT NULL,
  post_id bigint NOT NULL,
  check (inc in (-1, 1)),
  PRIMARY KEY (id),
  FOREIGN KEY (user_id) REFERENCES "user"(id),
  FOREIGN KEY (post_id) REFERENCES post(id)
);