# ASU Class Waitlist
The ASU class waitlist program is a system that allows you to create a profile and add classes to wishlist. The service will then email the user when the seating opens up.

## Setup

### With Docker
To make it as easier for any of our peer reviewers to get it up and running. First clone this directory by running:
```bash
git clone gh repo clone magnoliapena/cse412proj
```
Then, make sure that you have `docker-compose` installed then run:
```bash
docker compose up
```
Then, cd into the cliend directory like:
```bash
cd client
```
Then with npm installed, run:
```bash
npm run start
```
This should redirect you to a browser tab with our application running! 

### Without Docker
If you want to run the backend outside of a container you can start the database by running
```bash
docker compose up postgres-server
```
This will start just the database, in another terminal instance, navigate to the `server` directory and run:
```bash
cargo run
```
This will start the backend on localhost port 8080 Then with npm installed, run:
```bash
npm run start
```
This should redirect you to a browser tab with our application running! 

<br>

Please message any of the maintainers if you have any questions!
