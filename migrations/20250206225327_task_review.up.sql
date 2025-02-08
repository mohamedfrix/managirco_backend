CREATE TABLE task_review (
    id SERIAL PRIMARY KEY ,
    reviewer_id SERIAL NOT NULL ,
    task_submission_id SERIAL NOT NULL ,
    review_message TEXT NOT NULL ,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() ,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() ,

    FOREIGN KEY (reviewer_id) REFERENCES task_reviewer(id) ON DELETE CASCADE ,
    FOREIGN KEY (task_submission_id) REFERENCES task_submission(id) ON DELETE CASCADE
);