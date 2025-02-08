CREATE TABLE collaboration_members (
    id SERIAL PRIMARY KEY ,
    membership_id SERIAL NOT NULL ,
    collaboration_id SERIAL NOT NULL ,
    collaboration_role_id SERIAL NOT NULL ,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    FOREIGN KEY (collaboration_id) REFERENCES collaboration(id) ON DELETE CASCADE ,
    FOREIGN KEY (membership_id) REFERENCES membership(id) ON DELETE CASCADE ,
    FOREIGN KEY (collaboration_role_id) REFERENCES collaboration_role(id) ON DELETE CASCADE
);