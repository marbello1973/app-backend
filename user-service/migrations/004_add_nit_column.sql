-- Agregar columna nit a la tabla users
ALTER TABLE users ADD COLUMN nit VARCHAR(50) UNIQUE;

-- Crear índice para mejor performance
CREATE INDEX IF NOT EXISTS idx_users_nit ON users(nit);

-- Actualizar usuarios existentes con valores de nit
UPDATE users SET 
    nit = CASE 
        WHEN username = 'admin' THEN '900123456-1'
        WHEN username = 'johndoe' THEN '800987654-2'
        WHEN username = 'janedoe' THEN '700456789-3'
        ELSE NULL
    END;







