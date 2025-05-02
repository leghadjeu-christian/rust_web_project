-- Add up migration script here
DROP TABLE IF EXISTS files;

 CREATE TYPE status_enum AS enum('uploaded','compressed', 'compressing', 'compression_failed');


CREATE TABLE files (
  id SERIAL PRIMARY KEY,
  file_ref VARCHAR(255) NOT NULL,
  status status_enum DEFAULT 'uploaded',  
  compressed_file_ref  VARCHAR(255),
  uploaded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);