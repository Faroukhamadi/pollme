-- Add up migration script here
CREATE TABLE IF NOT EXISTS public.vote (
  id serial NOT NULL,
  -- change this to the correct type, int maybe?
  inc bigint NOT NULL,
  created_at timestamp without time zone NOT NULL DEFAULT NOW(),
  user_id bigint NOT NULL,
  post_id bigint NOT NULL,
  check (inc in (-1, 1)),
  PRIMARY KEY (id),
  FOREIGN KEY (user_id) REFERENCES "user"(id),
  FOREIGN KEY (post_id) REFERENCES post(id),
  CONSTRAINT vote_user_post_unique UNIQUE (user_id, post_id)
);

CREATE
OR REPLACE FUNCTION toggle_vote(inc NUMERIC, uid NUMERIC, pid NUMERIC,) RETURNS NUMERIC AS $ $ DECLARE row_exists NUMERIC;

BEGIN
SELECT
  1 INTO row_exists
FROM
  vote
WHERE
  user_id = uid
  and post_id = pid;

IF (row_exists > 0) THEN
DELETE FROM
  vote
WHERE
  user_id = uid
  and post_id = pid;

RETURN 0;

ELSE
INSERT INTO
  vote(inc, user_id, post_id)
VALUES
  (inc, uid, pid);

RETURN 1;

END IF;

END;

$ $ LANGUAGE plpgsql;