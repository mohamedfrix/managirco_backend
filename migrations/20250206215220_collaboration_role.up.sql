CREATE TABLE collaboration_role (
    id SERIAL PRIMARY KEY ,
    role_name VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() ,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);