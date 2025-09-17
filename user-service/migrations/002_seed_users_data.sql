-- Agregar las columnas faltantes a la tabla users
ALTER TABLE users 
ADD COLUMN IF NOT EXISTS first_name VARCHAR(50),
ADD COLUMN IF NOT EXISTS last_name VARCHAR(50),
ADD COLUMN IF NOT EXISTS is_active BOOLEAN DEFAULT TRUE,
ADD COLUMN IF NOT EXISTS is_admin BOOLEAN DEFAULT FALSE;

-- Actualizar los usuarios existentes con valores por defecto
UPDATE users SET 
    first_name = COALESCE(first_name, 'Unknown'),
    last_name = COALESCE(last_name, 'User'),
    is_active = COALESCE(is_active, TRUE),
    is_admin = COALESCE(is_admin, FALSE)
WHERE first_name IS NULL OR last_name IS NULL OR is_active IS NULL OR is_admin IS NULL;

-- Hacer que first_name y last_name no sean nulos después de poblarlos
ALTER TABLE users 
ALTER COLUMN first_name SET NOT NULL,
ALTER COLUMN last_name SET NOT NULL; 
