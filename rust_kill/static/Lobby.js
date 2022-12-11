
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
  location.assign('Room.html#Top');
}


let hForm = document.getElementById('hostform');
if(hForm) {
  hForm.addEventListener("submit", (e) => {
    //e.preventDefault();
    let data = new FormData(hForm);
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
} else{
  console.log("Host Form Failed");
}

  let cForm = document.querySelector('#clientform');

  if (cForm){
    cForm.addEventListener("submit", (e) => {
      //e.preventDefault();
      console.log("hi")
      let data = new FormData(cForm);
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
  } else{
    console.log("Client Form Failed");
  }
  

function ClientInfoSubscribe(uri) {
  var retryTime = 1000;
  function Connect(uri) {
    const events = new EventSource(uri);
    events.addEventListener("message", (ev) => {
      console.log(ev);
      const msg = JSON.parse(ev.data);
      if (!"username" in msg && !"room_name" in msg && !"client_addr" in msg && !"idx" in msg) return;
      localStorage.clear();
      localStorage.setItem('room_name', msg.room_name);
      localStorage.setItem('username', msg.username);
      localStorage.setItem('client_addr', msg.client_addr);
      localStorage.setItem('idx', msg.idx);
      ChangePage();
    });

    events.addEventListener("open", () => {
      console.log(`connected to event stream at ${uri}`);
      ChangePage();
      if (window.localStorage.length != 0) {
        retryTime = 1;
      }
    });

    events.addEventListener("error", () => {
      events.close();
      let timeout = retryTime;
      //retryTime = Math.min(64, retryTime * 2);
      console.log(`connection lost. attempting to reconnect in ${timeout}s`);
      setTimeout(() => Connect(uri), (() => timeout * 1000)());
    });
  }
  Connect(uri);
}


ClientInfoSubscribe("/clientInfo");
