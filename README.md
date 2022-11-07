# RustKill
[Gamestate & Description Doc](https://docs.google.com/document/d/1cfV9vRisjSao56QPLooyyAIHvg3fE0QyBqc-j96xIwU/edit)

**Team MemoryLeak**: Yichen Liu (yichen32), (pg22), Weijia Zhang (weijia4)

## Project Introduction

*Motivation*: To learn about the technical skills required in practical multithreading and chatroom servers, team collaboration in coding, as well as deployment of rust projects. 

*Goal*: Chatroom that supports multiplayer and creation and running of multiple simultaneous chatrooms based on similar rules to "Werewolves of Miller's Hollow"

*Short Description*: Multithreaded chat server/client and chatrooms for a mystery game similar to the "Werewolves of Miller's Hollow."

## Game Description
**Roles**
1. Ruster (Killers)
2. Vagrants (citizens with various roles) 
  1. 

## Technical Overview

**Concepts**
- [ ] Multithreading
- [ ] Chat server
- [ ] Docker/server rust deployment

**Libraries and Tools Used**
* Tokio-rs: Asynchronous networked rust application

**Chat Server System**

* `Struct Message`: Convey Properties such as text, userID, time that needed for sending a message
* `Class User`: UserName, UserID and other properties that needed for a player’
* `Class Event()`: Used for build channel<Message>, addListener(), receive and send message.

* Class Chatroom: a Chatroom class that contains everything needed for a chatroom
  * `WaitForPlayers()`
  * `Host()`
  * `KickPlayer(Player player)`
  * `InvitePlayer(Player player)`

* `Class ChatroomManager`: Used for managing, creating, searching and deleting multiple chat rooms. Use a map< key: int roomID,  value: Chatroom room> to manage them.
  * `Create(Chatroom room)`
  * `Delete(Chatroom room)`
  * `Search(int roomID)`
  * `Join(Chatroom room)`
* `Class DatabaseInteraction`: A class for interacting with databases

**Game Loop System**
  
* Game Loop
* Class GameState
* Role Assign
* `Struct AbstractRole`: `int id`, `Player&`, `Chatroom&`, `bool alive`
* `Struct VagrantRole` extends `AbstractRole: xxx`
* `Struct RusterRole` extends `AbstractRole: xxx`
* Role Specific Chat (Use tag to set visibility)
* Different roles process
* Win/Lose Condition/Detection

  
**Additional Features**
* UI: Menu, RoleImage, VoteEffect, RoomBackground
* Voice Messages

  
## Development Plan
  
| Date | Task |
| ------------- | ------------- |
| November 15      | Tasks up to Chatroom                       |
| November 30       |  Complete Chat Server System and Game Loop  |
| December 5     |  Finish Game Loop System | 
 

## Expected Challenges

**Project Oriented**
* Group member availability and cohesion
* Limited efficient use of time in between difficult STEM courses, midterms, finals. 
  
  
**Technical**
* Working with new rust crates (Tokio...)
* Docker and server deployment and testing
  



