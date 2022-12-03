CREATE TABLE asu_user (
  UserId text PRIMARY KEY,
  Password text,
  Location text,
  Username text,
  Major text
);

CREATE TABLE class (
    Units integer,
    Dates text,
    Status integer,
    StartTime text,
    EndTime text,
    Days text,
    Location text,
    Instructor text[],
    Course text,
    ClassId integer primary key,
    Session text,
    Term integer,
    Title text
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
  ClassId int references class
);


CREATE TABLE requirements (
    ClassId integer references class,
    Prerequisites text[]
);

INSERT INTO class (Units, Dates, Status, StartTime, EndTime, Days, Location, Instructor, Course, ClassId, Session, Term, Title) VALUES (3, '', 3, '', '', '', '', NULL, '', 42, '', 2214, '');
