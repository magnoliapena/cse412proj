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
  classes text[],
  term integer
);

CREATE TABLE wishlist (
  UserId text,
  ClassListId text,
  PriorityRanking integer,
  AddedDate date
);
 

CREATE TABLE taken (
  UserId text references asu_user,
  taken_ClassId integer, 
  taken_Term integer,
  Foreign key (taken_ClassId, taken_Term) references class
);


CREATE TABLE requirements (
  Prerequisites text[],
  req_ClassId integer, 
  req_Term integer,
  Foreign key (req_ClassId, req_Term) references class
);
