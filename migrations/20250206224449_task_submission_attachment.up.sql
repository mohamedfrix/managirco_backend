CREATE TABLE task_submission_attachment (
    id SERIAL PRIMARY KEY ,
    task_submission_id SERIAL NOT NULL ,
    file_path TEXT NOT NULL ,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() ,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() ,

    FOREIGN KEY (task_submission_id) REFERENCES task_submission(id) ON DELETE CASCADE
);