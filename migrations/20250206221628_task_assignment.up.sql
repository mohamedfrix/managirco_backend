CREATE TABLE task_assignment (
    id SERIAL PRIMARY KEY ,
    member_id SERIAL NOT NULL ,
    task_id SERIAL NOT NULL ,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() ,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() ,

    FOREIGN KEY (member_id) REFERENCES collaboration_members(id) ON DELETE CASCADE ,
    FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE CASCADE
);