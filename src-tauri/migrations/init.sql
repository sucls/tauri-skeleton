create table if not exists email(
    id INTEGER  PRIMARY KEY  AUTOINCREMENT,
    address text(128) not null,
    password Text(256) not null,
    kind Text(12),
    status Text(2)
);

create table if not exists email_message(
    id INTEGER  PRIMARY KEY  AUTOINCREMENT,
    subject Text(256) not null,
    source Text(128) not null,
    target Text(128) not null,
    cc Text(256),
    bcc Text(256),
    reply_to Text(256),
    category Text(56),
    folder Text(8),
    receive_date Date,
    send_date Date,
    text Text(1024),
    html Text(1024)
);

create table if not exists attachment(
    id INTEGER  PRIMARY KEY  AUTOINCREMENT,
    message_id int not null,
    name text(128) not null,
    kind Text(16) not null,
    path Text(256),
    data BLOB
);

create table if not exists folder(
    id INTEGER  PRIMARY KEY  AUTOINCREMENT,
    email_id int,
    name text(128),
    icon text(56),
    sort int
);

create table if not exists email_server(
    id INTEGER  PRIMARY KEY  AUTOINCREMENT,
    provider text(16),
    protocol text(8),
    host text(56),
    port int,
    ssl_port int
);
