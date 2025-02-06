CREATE TABLE membership (
    id SERIAL PRIMARY KEY ,
    user_id UUID NOT NULL ,
    department_id SERIAL NOT NULL ,
    role_id SERIAL NOT NULL ,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (department_id) REFERENCES department(id) ON DELETE CASCADE ,
    FOREIGN KEY (role_id) REFERENCES membership_role(id) ON DELETE CASCADE ,

    UNIQUE (user_id, department_id)
);