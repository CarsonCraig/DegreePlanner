CREATE TABLE terms (
  id SERIAL PRIMARY KEY,
  course_plan_id INTEGER NOT NULL,
  name VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  FOREIGN KEY (course_plan_id) REFERENCES course_plans (id)
);
