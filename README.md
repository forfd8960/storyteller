# Story Teller is A Service to create and tell stories

## The Architecture

![architecture](storyteller_architecture.png)

## The Module - Code Structure

```sh
src
├── lib.rs
├── main.rs
├── models
│   ├── mod.rs
│   ├── story.rs
│   └── user.rs
├── server
│   └── mod.rs
└── service
    ├── mod.rs
    ├── story_service.rs
    └── user_service.rs
```

* models => data storage
* service => apply business logic to data storage(implment logic for each endpoint)
* server handles:
  call service.
  set service methods to a specific router.
  auth.
  tracing
  log
  rate limit
  etc.

* main:
   call server and run the service listening on a TCP port.
