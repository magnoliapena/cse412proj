CREATE TABLE asu_user (
  UserId text PRIMARY KEY,
  Username text,
  Password text,
  Email text,
  Location text,
  Major text
);

CREATE TABLE class (
    ClassId integer,
    Title text,
    Units integer,
    Dates text,
    Status integer,
    Days text,
    StartTime text,
    EndTime text,
    Instructor text[],
    Location text,
    Course text,
    Session text,
    Term integer,
    primary key(ClassId, Term)
);


CREATE TABLE class_list (
  ClassListId text PRIMARY KEY,
  term integer
);

CREATE TABLE class_list_relationship (
  ClassListId text,
  Foreign key (ClassListId) references class_list(ClassListId),
  ClassId integer, 
  Term integer,
  Foreign key (ClassId, Term) references class(ClassId, Term)
);

CREATE TABLE wishlist (
  UserId text,
  Foreign key (UserId) references asu_user(UserId),
  ClassListId text,
  Foreign key (ClassListId) references class_list(ClassListId)
);
 

CREATE TABLE takenlist (
  UserId text,
  Foreign key (UserId) references asu_user(UserId),
  ClassListId text,
  Foreign key (ClassListId) references class_list(ClassListId)
);


CREATE TABLE requirements (
  Prerequisites text[],
  req_ClassId integer, 
  req_Term integer,
  Foreign key (req_ClassId, req_Term) references class
);
