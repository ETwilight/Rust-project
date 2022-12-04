


function HostInput() {
    let form = document.querySelector('#hostform');
    form.addEventListener("submit", (e)=>{
        //e.preventDefault();
        let data = new FormData(form);
        var object = {};
        data.forEach(function(value, key){
        object[key] = value;
        });
        var jsondat = JSON.stringify(object);
        console.log(jsondat);
      
            fetch("/playerInfo", {
              method: "POST",   
              body: new URLSearchParams({ username, serverIP, serverIP}),
            }).then((response) => {
              if (response.ok) console.log("OK");
            });
          
        return;
        
    })
} 

function ClientInput() {
    let form = document.querySelector('#clientform');
    form.addEventListener("submit", (e)=>{
        //e.preventDefault();
        let data = new FormData(form);
        var object = {};
        data.forEach(function(value, key){
        object[key] = value;
        });
        var jsondat = JSON.stringify(object);
        console.log(STATE.connected);
        console.log("GET STATUS: %s ", GetStatus());
            fetch("/playerInfo", {
              method: "POST",
              body: new URLSearchParams({ username, clientIP, serverIP }),
            }).then((response) => {
              if (response.ok) console.log("OK");
            });
        return;
    })

} 

//console.log(GetStatus());

