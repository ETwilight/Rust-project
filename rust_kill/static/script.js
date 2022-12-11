
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

var content = document.getElementById("content"); 
var username = "guest";

const id = parseInt(localStorage.idx);
var profile = document.getElementById("showid");
profile.innerHTML = "<b> CURRENT USER ID: " + id + "<b>";

const VoteEventType = {
  Kill: 'Kill',
  WerewolfGiveUp: "WerewolfGiveUp",
  Poison: "Poison",
  Antidote: "Antidote",
  WitchGiveUp: "WitchGiveUp",
  Reveal: "Reveal",
  Vote: "Vote",
}

const RoleType = {
  Civilian: "Civilian",
  Wolf: "Wolf",
  Witch: "Witch",
  Prophet: "Prophet",
  Undecided: "Undecided",
}

const AliveType = {
  Alive: "Alive",
  Dead: "Dead",
  Wound: "Wound",
}

const WinType = {
  Undecided: "Undecided",
  WerewolfWin: "WerewolfWin",
  CivilianWin: "CivilianWin",
  Draw: "Draw",
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

var room = {
  room_name: "",
  players : [],
  messages : [],
}
 
var role;

//refreshing content/////////
var STATE = {
  currentRoom: "rust_kill",
  rooms: {}, //A dictionary
  connected: false,
}

var content = document.getElementById("content");  
function scrollToBottom() {
    setTimeout(function(){
        content.scrollTop = content.scrollHeight;
    }, 50);   
}

function RoomSubscribe(uri) {
  var retryTime = 1;
  function Connect(uri) {
    const events = new EventSource(uri);
    events.addEventListener("message", (ev) => {
      const roomjson = JSON.parse(ev.data);
      console.log("decoded data", JSON.stringify(roomjson));
      localStorage.setItem("roomjson", roomjson);
      messagesDiv.innerHTML = "";
      room.messages = new Array();
      roomjson['messages'].forEach(function(val){
        console.log(val);
        var rm = room.messages;
        rm.push(val);
        room.messages = rm;
      });
   
      room.messages.forEach(function(val){
        AddMessage(roomjson["room_name"], val.username, 
          val.message, true);
          scrollToBottom();
      });
      
      console.log(room.messages);
      if(localStorage.getItem("idx") == null){
        AssignPlayerid(roomjson);
      }
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
  newMessageForm.addEventListener("submit", scrollToBottom);
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
function AddMessage(room_, username, message, push = false) {
  if (push) {
    console.log(STATE.rooms[room_]);
    STATE.rooms[room_].push({ username, message });
  }

  if (STATE.currentRoom == room_) {
    var node = messageTemplate.content.cloneNode(true);
    newMessageForm.addEventListener("submit", scrollToBottom);
    node.querySelector(".message .username").textContent = username;
    node.querySelector(".message .username").style.color = HashColor(username);
    node.querySelector(".message .text").textContent = message;
    newMessageForm.addEventListener("submit", scrollToBottom);
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
      if (!"message" in msg || !"room_name" in msg || !"username" in msg) return;
      AddMessage(msg.room_name, msg.username, msg.message, true);
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
      AddMessage("rust_kill", msg.username, msg.username+" has joined the chatroom", true);
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

function AddMessageEventListener(){
    // Set up the new message handler.
    newMessageForm.addEventListener("submit", (e) => {
      e.preventDefault();
      const message = messageField.value;
      if (!message) return;
  
      if (STATE.connected) {
        fetch("/room/message", {
          method: "POST",
          body: new URLSearchParams({id, message}),
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
//post events

function WolfListener(){
  var form = document.getElementById('killoption');
  var confirm = document.getElementById('killconfirm')
  confirm.addEventListener("submit", () => {
      var event_type, target_id;
      event_type = VoteEventType.Kill;
      target_id = form.value
      if (form.value == 'None'){
          event_type = VoteEventType.WerewolfGiveUp;
          target_id = 7;
      var voter_id = id;
      fetch("/game/event", {
          method: "POST",
          body: new URLSearchParams({event_type, voter_id, target_id}),
      }).then((response) =>{
          if (response.ok) console.log("Wolf Event Sent");
      });
      return;
  }})
}

function ProphetListener(){
  var form = document.getElementById('revealoption');
  var confirm = document.getElementById('revealconfirm');
  confirm.addEventListener("submit", () => {
      var target_id = form.value;
      if (form.value == 'None'){
          target_id = 7;
      }
      var voter_id = id;
      var event_type = VoteEventType.Reveal;
      fetch("/game/event", {
          method: "POST",
          body: new URLSearchParams({event_type, voter_id, target_id}),
      }).then((response) =>{
          if (response.ok) console.log("Prophesizing Event Sent");
      });
      return;
  })
}

function WitchListener(){
  var form = document.getElementById('witchoption');
  var drug = document.getElementById('drug');
  var confirm = document.getElementById('witchconfirm');
  confirm.addEventListener("submit", () => {
      var event_type;
      var target_id = form.value;
      if (drug.value == 'antidote'){
          event_type = VoteEventType.Antidote;
      }else if (drug.value == 'poison'){
          event_type = VoteEventType.Poison
      }else if (drug.value == 'None' || form.value == 'None'){
          event_type = VoteEventType.WitchGiveUp;
          target_id = 7; 
      }
      var voter_id = id;
      fetch("/game/event", {
          method: "POST",
          body: new URLSearchParams({event_type, voter_id, target_id}),
      }).then((response) =>{
          if (response.ok) console.log("Witching Event Sent");
      });
      return;
  })
}

function VoteListener(){
  var form = document.getElementById('voteoption');
  var confirm = document.getElementById('voteconfirm');
  confirm.addEventListener("submit", () => {
      var target_id = form.value;
      if (form.value == 'None'){
          target_id = 7;
      }
      var voter_id = id;
      var event_type = VoteEventType.Vote;
      fetch("/game/event", {
          method: "POST",
          body: new URLSearchParams({event_type, voter_id, target_id}),
      }).then((response) =>{
          if (response.ok) console.log("Normal Voting Event Sent");
      });
      return;
  }) 
}

function EndSpeakTurnListener(){
  var endspeak = document.getElementById("endspeakturn");
  endspeak.addEventListener("submit", () => {
      fetch("/game/endspeak", {
          method: "POST",
          body: new URLSearchParams({id}),
      }).then((response) =>{
          if (response.ok) console.log("End Current Speaking Turn");
      });
      return;
  }) 
}

function dead(){
  var others = document.getElementsByClassName("playerStatus");
  for (var i = 0; i < others.length; i++) {
    others[i].classList.remove("speaking");
    others[i].classList.remove("mute");
    others[i].classList.add("dead");
  }
}

function muted(){
  var others = document.getElementsByClassName("playerStatus");
  for (var i = 0; i < others.length; i++) {
    others[i].classList.remove("dead");
    others[i].classList.remove("speaking");
    others[i].classList.add("mute");
  }
}

function speaking(){
  var others = document.getElementsByClassName("playerStatus");
  for (var i = 0; i < others.length; i++) {
    others[i].classList.remove("mute");
    others[i].classList.remove("dead");
    others[i].classList.add("speaking");
  }
}

document.getElementById("myRange").onchange = function() {
  for(let i = 1;i<=this.value;){
    document.querySelector(".player"+(i).toString()).style.visibility = "visible";
  i++;
  }
}

function replace(turnparam) {
  empty = document.getElementById("empty");
  switch (turnparam) {
      case 1:
          empty.innerHTML = "<div class = 'vote' id = 'wolfkill'> Select Target <form> <select id='killoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick = \"WolfListener()\" id = 'killconfirm'>Confirm</button> </form> Vote Out: <form> <select id='voteoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick = \"votesubmit()\" id = 'voteconfirm'>Confirm</button> </form> </div>";
          break;
      case 3:
          empty.innerHTML = "<div class = 'vote' id = 'reveal'> Select Target <form> <select id='revealoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick = \"ProphetListener()\" id = 'revealconfirm'>Confirm</button> </form> Vote Out: <form> <select id='voteoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick = \"votesubmit()\" id = 'voteconfirm'>Confirm</button> </form> </div>";
          break;
      case 2:
          empty.innerHTML = "<div class = 'vote' id = 'witchchoice'> Choose: <div> <form> <select id = 'drug'> <option value = 'antidote' type ='submit'>Antidote</option> <option value = 'poison' type ='submit'>Poison</option> <option value = 'None' type ='submit'>None</option> </select> </form> </div> <form> <select id='witchoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick=\"WitchListener()\" id = 'witchconfirm'>Confirm</button> </form> Vote Out: <form> <select id='voteoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick = \"votesubmit()\" id = 'voteconfirm'>Confirm</button> </form> </div> ";
          break;
      case 4:
          empty.innerHTML = "<div class = 'vote' id = 'civilianchoice'> Vote Out: <form> <select id='voteoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick = \"VoteListener()\" id = 'voteconfirm'>Confirm</button> </form> </div>";
      default:
          break;
  }
}

/* => GAME LOOP*/ 
function mute(){
  var w = document.getElementById('disabled').style.visibility = "visible";
  document.querySelector('.textmessage').disabled = true;
}

function unmute(){
  var w = document.getElementById('disabled').style.visibility = "hidden";
  document.querySelector('.textmessage').disabled = false;
}


function AssignPlayerid(roomjson){
  let count = 0;
  roomjson.players.forEach((player) => {
    if(player.id != '7'){
      count++;
    }
    let i = 1;
    for(;i<=count;){
      document.querySelector(".player"+(i).toString()).style.visibility = "visible";
      i++;
    }
  })
  localStorage.setItem("idx", count);
}

/*Utilities*/




// function findTurn(){
//   console.log("");
//   switch(room.game_state.TurnType){
//     case WolfTurn:

//       replace(1);
//       break;
//     case WitchTurn:

//       replace(2);
//       break;
//     case ProphetTurn:
//       //if is prophet
//       replace(3);
//       break;
//     case VoteTurn:
//       //disable false
//       break;
//     case SpeakTurn:
//       //if id matches
//       unmute();
//       break;
//     default:
//       break;
//   }
// }

function Init() {
  // Initialize the room.
  AddRoom("rust_kill");
  ChangeRoom("rust_kill");
  AddMessage("rust_kill", "Rocket", "Howdy! Open another browser tab, send a message.", true);

  AddMessageEventListener();
  AddRoomListener();
  // findTurn();


  // Subscribe to server-sent events.
  RoomSubscribe("/event/room")
  
}


Init();