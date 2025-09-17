-- Insertar usuarios con todos los campos
INSERT INTO users (username, email, password, first_name, last_name, is_active, is_admin) 
VALUES 
    ('admin', 'admin@example.com', 'admin123', 'Admin', 'User', TRUE, TRUE),
    ('johndoe', 'john@example.com', 'john123', 'John', 'Doe', TRUE, FALSE),
    ('janedoe', 'jane@example.com', 'jane123', 'Jane', 'Doe', TRUE, FALSE)
ON CONFLICT (username) 
DO UPDATE SET
    first_name = EXCLUDED.first_name,
    last_name = EXCLUDED.last_name,
    is_active = EXCLUDED.is_active,
    is_admin = EXCLUDED.is_admin,
    updated_at = CURRENT_TIMESTAMP;
