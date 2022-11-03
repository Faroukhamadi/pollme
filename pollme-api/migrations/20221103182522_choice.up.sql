-- Add up migration script here
CREATE TABLE IF NOT EXISTS public.choice (
  id serial NOT NULL,
  choice character varying NOT NULL,
  created_at timestamp without time zone NOT NULL DEFAULT NOW(),
  post_id bigint NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (post_id) REFERENCES post(id)
);