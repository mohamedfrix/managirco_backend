CREATE TABLE task (
    id SERIAL PRIMARY KEY ,
    collaboration_id SERIAL NOT NULL ,
    title VARCHAR(255) NOT NULL ,
    description TEXT ,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() ,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() ,

    FOREIGN KEY (collaboration_id) REFERENCES collaboration(id) ON DELETE CASCADE
);