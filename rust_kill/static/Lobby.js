
function openClient() {
  document.getElementById("f2").style.display = "block";
}

function openHost() {
  document.getElementById("f1").style.display = "block";
}

function closeClient() {
  document.getElementById("f2").style.display = "none";
}

function closeHost() {
  document.getElementById("f1").style.display = "none";
}

function ChangePage() {
  location.replace('Room.html#Top');
}

function HostInput() {
  let form = document.querySelector('#hostform');
  form.addEventListener("submit", (e) => {
    //e.preventDefault();
    let data = new FormData(form);
    var object = {};
    data.forEach(function (value, key) {
      object[key] = value;
    });
    var jsondata = JSON.stringify(object);
    var parsedjson = JSON.parse(jsondata);
    const username = parsedjson["username"];
    const serverip = parsedjson["serverip"];
    console.log("username: "+username);
    console.log("serverip: "+serverip);
    fetch("/room/host", {
      method: "POST",
      body: new URLSearchParams({username, serverip}),
    }).then((response) => {
      if (response.ok) console.log("Host Form Sent");
    });
  })

}

function ClientInput() {
  let form = document.querySelector('#clientform');
  form.addEventListener("submit", (e) => {
    //e.preventDefault();
    let data = new FormData(form);
    var object = {};
    data.forEach(function (value, key) {
      object[key] = value;
    });
    var jsondata = JSON.stringify(object);
    var parsedjson = JSON.parse(jsondata);
    const username = parsedjson["username"];
    const serverip = parsedjson["serverip"];
    console.log("username: "+username);
    console.log("serverip: "+serverip);
    fetch("/room/join", {
      method: "POST",
      body: new URLSearchParams({username, serverip}),
    }).then((response) => {
      if (response.ok) console.log("Client Form Sent");
    });
  })

}



function ClientInfoSubscribe(uri) {
  var retryTime = 1;
  function Connect(uri) {
    const events = new EventSource(uri);
    events.addEventListener("message", (ev) => {
      const msg = JSON.parse(ev.data);
      console.log("decoded data", JSON.stringify(msg));
      if (!"username" in msg || !"room_name" in msg || !"client_addr" in msg || !"idx" in msg) return;
      localStorage.setItem('room_name', msg.room_name);
      localStorage.setItem('username', msg.username);
      localStorage.setItem('client_addr', msg.client_addr);
      localStorage.setItem('idx', msg.idx);
      ChangePage();
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


ClientInfoSubscribe("/clientInfo");