ALTER TABLE task ADD status TEXT CHECK (status in ('done', 'pending')) NOT NULL DEFAULT 'pending';
