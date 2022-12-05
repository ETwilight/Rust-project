//import GetStatus from "./script";

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
              body: new URLSearchParams({ room, username, message }),
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
        console.log(jsondat);

            fetch("/playerInfo", {
              method: "POST",
              body: new URLSearchParams({ room, username, message }),
            }).then((response) => {
              if (response.ok) console.log("OK");
            });
        return;
    })

} 

console.log(GetStatus());

