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

function HostInput() {
  localStorage.setItem('is_input', true);
  localStorage.setItem('idx', '0');
  let hForm = document.getElementById('hostform');
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
    console.log("username: " + username);
    console.log("serverip: " + serverip);
    fetch("/room/host", {
      method: "POST",
      body: new URLSearchParams({ username, serverip }),
    }).then((response) => {
      if (response.ok) console.log("Host Form Sent");
    });


  })

}

function ClientInput() {
  localStorage.setItem('is_input', true);
  let cForm = document.getElementById('clientform');
  if (cForm) {
    cForm.addEventListener("submit", (e) => {
      //e.preventDefault();
      let data = new FormData(cForm);
      var object = {};
      data.forEach(function (value, key) {
        object[key] = value;
      });
      var jsondata = JSON.stringify(object);
      var parsedjson = JSON.parse(jsondata);
      const username = parsedjson["username"];
      const serverip = parsedjson["serverip"];
      console.log("username: " + username);
      console.log("serverip: " + serverip);
      fetch("/room/join", {
        method: "POST",
        body: new URLSearchParams({ username, serverip }),
      }).then((response) => {
        if (response.ok) console.log("Client Form Sent");
      });
    })
  } else {
    console.log("Client Form Failed");
  }
}

function ClientInfoSubscribe(uri) {
  var retryTime = 1;
  function Connect(uri) {
    const events = new EventSource(uri);
    console.log(events);
    events.addEventListener("message", (ev) => {
      //ev.preventDefault();
      const msg = JSON.parse(ev.data);
      console.log(msg);
      localStorage.setItem('msgs', msg);
      //console.log("decoded data", JSON.stringify(msg));
      //if (!"username" in msg && !"room_name" in msg && !"client_addr" in msg && !"idx" in msg) return;
      localStorage.clear();
      localStorage.setItem('room_name', msg.room_name);
      localStorage.setItem('username', msg.username);
      localStorage.setItem('client_addr', msg.client_addr);
      localStorage.setItem('idx', msg.idx);
      ChangePage();
    });
    events.addEventListener("open", (ev) => {
      console.log(`connected to event stream at ${uri}`);
      console.log(ev);
      console.log(window.localStorage);
      //const msg = JSON.parse(ev.data);
      if (window.localStorage.getItem('is_input') == true) {
        ChangeRoom();
      }
    });

    events.addEventListener("error", () => {
      events.close();
      let timeout = retryTime;
      retryTime = Math.min(64, retryTime * 2);
      console.log(`connection lost. attempting to reconnect in ${timeout}s`);
      setTimeout(() => Connect(uri), (() => timeout * 1000)());
    });
  }

  Connect(uri);

}

function Init() {
  //localStorage.clear();
  ClientInfoSubscribe("/clientInfo");
}

Init();
