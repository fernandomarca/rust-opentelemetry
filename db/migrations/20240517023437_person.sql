-- migrate:up
create table person(
    id serial primary key,
    name varchar(255) not null,
    age int not null
);

insert into person(name, age) values('Alice', 30);
insert into person(name, age) values('Bob', 25);

-- migrate:down
drop table person;
