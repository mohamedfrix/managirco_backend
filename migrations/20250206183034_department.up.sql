CREATE TABLE department (
    id SERIAL PRIMARY KEY ,
    club_id SERIAL NOT NULL ,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    FOREIGN KEY (club_id) REFERENCES club(id) ON DELETE CASCADE,
    UNIQUE (name, club_id)
);