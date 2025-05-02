CREATE DATABASE socialmediadb
;
 CREATE TABLE users(user_id SERIAL NOT NULL, name VARCHAR(199) NOT NULL, user_password VARCHAR(199) NOT NULL, email VARCHAR(199) NOT NULL,number_of_followers INT DEFAULT 0, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);
CREATE TABLE posts(post_id SERIAL NOT NULL, title VARCHAR(199) NOT NULL, number_of_likes INT NOT NULL DEFAULT 0, number_of_views DECIMAL NOT NULL, user_id INT NOT NULL, FOREIGN KEY(user_id) REFERENCES users(user_id),created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);  
ALTER TABLE users ADD  PRIMARY KEY (user_id);
ALTER TABLE posts ADD  PRIMARY KEY (post_id);
CREATE TYPE gender_enum AS ENUM('male','female');
ALTER TABLE users ADD COLUMN gender gender_enum NOT NULL;
ALTER TABLE users ADD COLUMN date_of_birth TIMESTAMP NOT NULL;
CREATE TABLE comments(comment_id SERIAL NOT NULL, comment VARCHAR(199) NOT NULL, number_of_likes INT DEFAULT 0, post_id INT NOT NULL, FOREIGN KEY(post_id) REFERENCES posts(post_id), created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);  

