CREATE TABLE task_attachment (
    id SERIAL PRIMARY KEY ,
    task_id SERIAL NOT NULL ,
    file_path TEXT NOT NULL ,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() ,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() ,

    FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE CASCADE
);