-- Your SQL goes here
    CREATE TABLE events (
      id SERIAL PRIMARY KEY,
      title VARCHAR NOT NULL,
      description VARCHAR NOT NULL,
      link VARCHAR NOT NULL,
      kind VARCHAR NOT NULL
    );

    INSERT INTO events(title, description, link, kind) VALUES ('Link','Description for Link Event', 'www.google.com', 'meetup');
    INSERT INTO events(title, description, link, kind) VALUES ('Mario','Description for Link Event', 'www.google.com', 'meetup');
    INSERT INTO events(title, description, link, kind) VALUES ('Kirby','Description for Link Event', 'www.google.com', 'dinner');
    INSERT INTO events(title, description, link, kind) VALUES ('Ganondorf','Description for Link Event', 'www.google.com', 'planning');
    INSERT INTO events(title, description, link, kind) VALUES ('Bowser','Description for Link Event', 'www.google.com', 'meetup');
    INSERT INTO events(title, description, link, kind) VALUES ('Mewtwo','Description for Link Event', 'www.google.com', 'meetup');