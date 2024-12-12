CREATE TABLE Workflows (
    ID          VARCHAR(255) PRIMARY KEY,
    user_id     INT NOT NULL,
    data        VARCHAR(255) NOT NULL,
    selector    VARCHAR(255) NOT NULL,
    cron        VARCHAR(255) NOT NULL,
    lastupdated BIGINT,
    url         VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users(user_id)
)
