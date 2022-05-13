drop table if exists todo_list;
drop table if exists todo_item;

create table todo_list
(
    id    serial primary key,
    title varchar(255) not null
);

create table todo_item
(
    id           serial primary key,
    title        varchar(255) not null,
    checked      boolean      not null default false,
    todo_list_id integer      not null,
    foreign key (todo_list_id) references todo_list (id)
);

insert into todo_list (title) values ('Todo List 1'), ('Todo List 2');

insert into todo_item (title, todo_list_id) values ('Item 1', 1), ('Item 2', 1), ('Item 3', 2), ('Item 4', 2);