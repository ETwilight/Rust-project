
//variables////////////////////////////////////////////////////////////////

let roomListDiv = document.getElementById('room-list');
let messagesDiv = document.getElementById('messages');
let newMessageForm = document.getElementById('new-message');
let newRoomForm = document.getElementById('new-room');
let statusDiv = document.getElementById('status');

let roomTemplate = document.getElementById('room');
let messageTemplate = document.getElementById('message');
let messageField = newMessageForm.querySelector("#message");
let roomNameField = newRoomForm.querySelector("#name");

var username = "guest";

const Role = {
  Civilian: "Civilian",
  Wolf: "Wolf",
  Witch: "Witch",
  Prophet: "Prophet",
  Undecided: "Undecided",
}
const TurnType = {
  StartTurn: "StartTurn",
  WolfTurn: "WolfTurn",
  WitchTurn: "WitchTurn",
  ProphetTurn : "ProphetTurn",
  SpeakTurn : "SpeakTurn",
  VoteTurn: "VoteTurn",
  LastWordTurn: "LastWordTurn",
  EndTurn: "EndTurn",
}
var PlayerState = {
  turn: false,
  muted: true,
  speaking: false, 
}

var Turn = {
  turn_state : TurnType,
}

var player = {
  name : "",
  ip : "", 
  role: Role,
  player_state: PlayerState,
}

var room = {
  room_name: "",
  players : {}, //{'1': player, '2': player, '3': player, '4': player, '5': player, '6': player},
  game_state : Turn,
}

var STATE = {
  currentRoom: "rustkill",
  rooms: {}, //A dictionary
  connected: false,
}

var content = document.getElementById("content");  
function scrollToBottom() {
    setTimeout(function(){
        content.scrollTop = content.scrollHeight;
    }, 50);   
}
//不一定对建议检查一下
////////////////////////////////////////////////////////////////
function RoomSubscribe(uri) {
  var retryTime = 1;
  function Connect(uri) {
    const events = new EventSource(uri);
    events.addEventListener("message", (ev) => {
      const roomjson = JSON.parse(ev.data);
      console.log("decoded data", JSON.stringify(roomjson));
      if (!room_name || !players || !game_state in room) return;
      //initialize
      room.room_name = roomjson.room_name;
      for (let i = 0, emp = roomjson.players[i]; i < roomjson.players.length; ++i){
        room.players[emp.id] = emp;
      }
      room.game_state = roomjson.game_state;
      console.log("ROOM OBJECT: " + room);
    });

    events.addEventListener("open", () => {
      SetConnectedStatus(true);
      console.log(`connected to event stream at ${uri}`);
      retryTime = 1;
    });

    events.addEventListener("error", () => {
      SetConnectedStatus(false);
      events.close();

      let timeout = retryTime;
      retryTime = Math.min(64, retryTime * 2);
      console.log(`connection lost. attempting to reconnect in ${timeout}s`);
      setTimeout(() => Connect(uri), (() => timeout * 1000)());
    });
    console.log(events);
  }
 
  Connect(uri);
}
////////////////////////////////////////////////////////////////

// Generate a color from a "hash" of a string. Thanks, internet.
function HashColor(str) {
  let hash = 0;
  for (var i = 0; i < str.length; i++) {
    hash = str.charCodeAt(i) + ((hash << 5) - hash);
    hash = hash & hash;
  }

  return `hsl(${hash % 360}, 100%, 70%)`;
}

// Add a new room `name` and change to it. Returns `true` if the room didn't
// already exist and false otherwise.
function AddRoom(name) {
  if (STATE.rooms[name]) {
    ChangeRoom(name);
    return false;
  }

  var node = roomTemplate.content.cloneNode(true);
  var room = node.querySelector(".room");
  room.addEventListener("click", () => ChangeRoom(name));
  room.textContent = name;
  room.dataset.name = name;
  roomListDiv.appendChild(node);

  STATE.rooms[name] = [];
  ChangeRoom(name);
  return true;
}

// Change the current room to `name`, restoring its messages.
function ChangeRoom(name) {
  if (STATE.currentRoom == name) return;

  var newRoom = roomListDiv.querySelector(`.room[data-name='${name}']`);
  var oldRoom = roomListDiv.querySelector(`.room[data-name='${STATE.currentRoom}']`);
  if (!newRoom || !oldRoom) return;

  STATE.currentRoom = name;
  oldRoom.classList.remove("active");
  newRoom.classList.add("active");

  messagesDiv.querySelectorAll(".message").forEach((msg) => {
    messagesDiv.removeChild(msg)
  });

  STATE.rooms[name].forEach((data) => AddMessage(name, data.username, data.message))
}

// Add `message` from `username` to `room`. If `push`, then actually store the
// message. If the current room is `room`, render the message.
function AddMessage(room, username, message, push = false) {
  if (push) {
    STATE.rooms[room].push({ username, message })
  }

  if (STATE.currentRoom == room) {
    var node = messageTemplate.content.cloneNode(true);
    newMessageForm.addEventListener("submit", scrollToBottom);
    node.querySelector(".message .username").textContent = username;
    node.querySelector(".message .username").style.color = HashColor(username);
    node.querySelector(".message .text").textContent = message;
    messagesDiv.appendChild(node);
  }
}



// Subscribe to the event source at `uri` with exponential backoff reconnect.
function MessageSubscribe(uri) {
  var retryTime = 1;

  function Connect(uri) {
    const events = new EventSource(uri);
    events.addEventListener("message", (ev) => {
      console.log("raw data", JSON.stringify(ev.data));
      const msg = JSON.parse(ev.data);
      console.log("decoded data", JSON.stringify(msg));
      if (!"message" in msg || !"room" in msg || !"username" in msg) return;
      AddMessage(msg.room, msg.username, msg.message, true);
    });

    events.addEventListener("open", () => {
      SetConnectedStatus(true);
      console.log(`connected to event stream at ${uri}`);
      retryTime = 1;
    });

    events.addEventListener("error", () => {
      SetConnectedStatus(false);
      events.close();

      let timeout = retryTime;
      retryTime = Math.min(64, retryTime * 2);
      console.log(`connection lost. attempting to reconnect in ${timeout}s`);
      setTimeout(() => Connect(uri), (() => timeout * 1000)());
    });
  }

  Connect(uri);
}

// Subscribe to the event source at `uri` with exponential backoff reconnect.
function PlayerInfoSubscribe(uri) {
  var retryTime = 1;
  
  function Connect(uri) {
    const events = new EventSource(uri);
    events.addEventListener("message", (ev) => {
      console.log("raw data", JSON.stringify(ev.data));
      const msg = JSON.parse(ev.data);
      console.log("decoded data", JSON.stringify(msg));
      if (!"username" in msg || !"serverip" in msg) return;
      username = msg.username;
      AddMessage("rustkill", msg.username, msg.username+" has joined the chatroom", true);
    });

    events.addEventListener("open", () => {
      SetConnectedStatus(true);
      console.log(`connected to event stream at ${uri}`);
      retryTime = 1;
    });

    events.addEventListener("error", () => {
      SetConnectedStatus(false);
      events.close();

      let timeout = retryTime;
      retryTime = Math.min(64, retryTime * 2);
      console.log(`connection lost. attempting to reconnect in ${timeout}s`);
      setTimeout(() => Connect(uri), (() => timeout * 1000)());
    });
    console.log(events);
  }
 
  Connect(uri);
}

// OnLoad will sent post to rust when the javascript start
function OnLoad(){
  if (STATE.connected) {
    fetch("/message", {
      method: "POST",
      body: new URLSearchParams({ room, username, message }),
    }).then((response) => {
      if (response.ok) console.log("PageOnLoad");
    });
  }
}
// Set the connection status: `true` for connected, `false` for disconnected.
function SetConnectedStatus(status) {
  STATE.connected = status;
  statusDiv.className = (status) ? "connected" : "reconnecting";
}

function AddMessageListener(){
    // Set up the new message handler.
    newMessageForm.addEventListener("submit", (e) => {
      e.preventDefault();
  
      const room = STATE.currentRoom;
      const message = messageField.value;
      const visible_type = "All";
      if (!message || !username) return;
  
      if (STATE.connected) {
        fetch("/message", {
          method: "POST",
          body: new URLSearchParams({ room, username, message, visible_type }),
        }).then((response) => {
          if (response.ok) messageField.value = "";
        });
      }
    })
}


function AddRoomListener(){
   // Set up the new room handler.
   newRoomForm.addEventListener("submit", (e) => {
    e.preventDefault();

    const room = roomNameField.value;
    if (!room) return;

    roomNameField.value = "";
    if (!AddRoom(room)) return;

    AddMessage(room, "Rocket", `Look, your own "${room}" room! Nice.`, true);
  })
}

function GetStatus(){
  return STATE.connected;
}

function Init() {
  // Initialize the room.
  AddRoom("rustkill");
  ChangeRoom("rustkill");
  AddMessage("rustkill", "Rocket", "Howdy! Open another browser tab, send a message.", true);

  AddMessageListener();
  AddRoomListener();

  // Subscribe to server-sent events.
  MessageSubscribe("/message/event");
  PlayerInfoSubscribe("/playerInfo/event")
  
}

// export {AddMessage, GetStatus};


Init();