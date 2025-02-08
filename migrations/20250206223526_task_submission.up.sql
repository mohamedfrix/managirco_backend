CREATE TABLE task_submission (
    id SERIAL PRIMARY KEY ,
    task_assignment_id SERIAL UNIQUE NOT NULL ,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() ,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() ,

    FOREIGN KEY (task_assignment_id) REFERENCES task_assignment(id) ON DELETE CASCADE
);