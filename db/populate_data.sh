echo "adding classes..."
psql -d postgres -q -c "copy class from '/var/lib/tabledata/classes.csv' with delimiter as '|' NULL '';"
echo "fixing data"
psql -d postgres -q -c "UPDATE class SET days = 'n/a' WHERE days IS NULL;"
psql -d postgres -q -c "UPDATE class SET instructor = '{"n/a"}' WHERE instructor IS NULL;"
psql -d postgres -q -c "UPDATE class SET endtime = 'n/a' WHERE endtime IS NULL;"
psql -d postgres -q -c "UPDATE class SET starttime = 'n/a' WHERE starttime IS NULL;"
echo "running tests"
psql -d postgres -q -c "SELECT count(*) FROM asu_user;"
psql -d postgres -q -c "SELECT count(*) FROM class;"
psql -d postgres -q -c "SELECT count(*) FROM class_list;"
