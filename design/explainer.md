# Optic Mart Architecture  
Optic Mart is based on modern software architecture.

## Backend  
Optic Mart...  
1. consists of a handful of autonomous  _microservices_.
2. is _event-driven_.
3. segregates reads from writes _(CQRS)_.
4. exposes and uses _REST API_s.

### Scenarios  
A _command_ goes through these phases:  
|#	|action																				|example			|
|:-----:|:-------------------------------------------------------------------------------------------------------------------------------------------------------------:|:-----------------------------:|
|1	|A command is issued by a client to the revelant endpoint which is the service-manager.										|POST api.optic-mart.com/user	|
|2	|Service-manager finds a revelant idle service and forwards the command to it.											|POST api.user1.optic-mart.com	|
|3	|The target service validates the request. It may have to issue a GET request to get the current state of the resource in question.				|is_request_valid() -> bool	|
|4	|If the request is valid, the target service generates a version tag. For new resources this is 1, for others it is previous + 1.				|generate_version_tag()	-> i64	|
|5	|For each affected entity, the target service requests appropriate event(s) to be stored in the event store with the generated version tag, entity id and type.	|POST event-store.optic-mart.com|
|6	|The event store validates the incoming request's version tag.													|is_version_valid() -> bool	|
|7	|If the version tag is valid, the event store stores the events with the metadata and emits them. It also successfully returns to the target service.		|raise_events(); 201 CREATED	|
|8	|Upon event store's successful return, the target service returns successfully to the service-manager.								|201 CREATED			|
|9	|Upon tagret service's successful return, the service-manager returns successfully to the client.								|201 CREATED			|

#### Branches  
##### Scenario 2a: A Revelant Idle Service Cannot Be Found.  
|#	|action								|example			|
|:-----:|:-------------------------------------------------------------:|:-----------------------------:|
|1	|Service-manager returns 503 Service Unavailable to the client.	|503 Service Unavailable	|

##### Scenario 4a: The Request Is Invalid.  
|#	|action										|example		|
|:-----:|:-----------------------------------------------------------------------------:|:---------------------:|
|1	|The target service returns a response in 400 range to the service-manager.	|400 Bad Request	|
|2	|The service manager returns the response it got to the client.			|400 Bad Request	|

##### Scenario 7a: The Version Tag Is Invalid.  
|#	|action										|example			|
|:-----:|:-----------------------------------------------------------------------------:|:-----------------------------:|
|1	|The event store returns 400 Bad Request to the target service.			|400 Bad Request		|
|2	|The target service retries from step 3 in the main scenarion.			|N/A				|

