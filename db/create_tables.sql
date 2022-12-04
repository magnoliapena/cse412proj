CREATE TABLE asu_user (
  UserId text PRIMARY KEY,
  Password text,
  Location text,
  Username text,
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

CREATE TABLE wishlist (
  UserId text,
  ClassListId text,
  PriorityRanking int,
  AddedDate date
);
 
CREATE TABLE class_list (
  ClassListId text PRIMARY KEY,
  Semester text
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
