
let roomListDiv = document.getElementById('room-list');
let messagesDiv = document.getElementById('messages');
let newMessageForm = document.getElementById('new-message');
let newRoomForm = document.getElementById('new-room');
let statusDiv = document.getElementById('status');

let roomTemplate = document.getElementById('room');
let messageTemplate = document.getElementById('message');

let messageField = newMessageForm.querySelector("#message");
let usernameField = newMessageForm.querySelector("#username");
let roomNameField = newRoomForm.querySelector("#name");


const PlayerState = {
  Alive: "Alive",
  Out: "Out",
  Leave: "Leave",
}
var STATE = {
  currentRoom: "lobby",
  rooms: {}, //A dictionary
  connected: false,
}


var player = {
  name: "Guest",
  ip: "",
  id: "0",
  status: PlayerState.Alive,
  isSpeaking: false, //if it's the player's turn, it will be true
}

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
      if (!"username" in msg || !"clientIP" in msg || !"serverIP" in msg) return;
      AddMessage("lobby", msg.username, msg.clientIP, true);
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
      const username = usernameField.value || "guest";
      if (!message || !username) return;
  
      if (STATE.connected) {
        fetch("/message", {
          method: "POST",
          body: new URLSearchParams({ room, username, message }),
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
  AddRoom("lobby");
  ChangeRoom("lobby");
  AddMessage("lobby", "Rocket", "Howdy! Open another browser tab, send a message.", true);

  AddMessageListener();
  AddRoomListener();

  // Subscribe to server-sent events.
  MessageSubscribe("/message/event");
  PlayerInfoSubscribe("/playerInfo/event")
  
}

// export {AddMessage, GetStatus};


Init();


