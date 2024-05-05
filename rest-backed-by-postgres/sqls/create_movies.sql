CREATE TABLE movies
(
    id         SERIAL PRIMARY KEY,
    movie_name VARCHAR(255),
    movie_year INT,
    rating     FLOAT
);

INSERT INTO movies (movie_name, movie_year, rating)
VALUES ('The Shawshank Redemption', 1994, 9.3),
       ('The Godfather', 1972, 9.2),
       ('The Dark Knight', 2008, 9.0),
       ('12 Angry Men', 1957, 9.0),
       ('Schindler''s List', 1993, 8.9);

INSERT INTO movies (movie_name, movie_year, rating) VALUES
                                                        ('Pulp Fiction', 1994, 8.9),
                                                        ('Forrest Gump', 1994, 8.8),
                                                        ('The Lion King', 1994, 8.5),
                                                        ('Leon: The Professional', 1994, 8.5);

INSERT INTO movies (movie_name, movie_year, rating) VALUES
                                                        ('The Shawshank Redemption', 1994, 9.3),
                                                        ('Speed', 1994, 7.2),
                                                        ('Dumb and Dumber', 1994, 7.3),
                                                        ('Four Weddings and a Funeral', 1994, 7.1);