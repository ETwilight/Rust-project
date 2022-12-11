const VoteEventType = {
    Kill: 'Kill',
    WerewolfGiveUp: "WerewolfGiveUp",
    Poison: "Poison",
    Antidote: "Antidote",
    WitchGiveUp: "WitchGiveUp",
    Reveal: "Reveal",
    Vote: "Vote",
  };

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
        var voter_id = parseInt(localStorage.getItem('idx'));
        fetch("/game/event", {
            method: "POST",
            body: new URLSearchParams({event_type, voter_id, target_id}),
        }).then((response) =>{
            if (response.ok) console.log("Wolf Event Sent");
        });
        return;
    })
}


function ProphetListener(){
    var form = document.getElementById('revealoption');
    var confirm = document.getElementById('revealconfirm');
    confirm.addEventListener("submit", () => {
        var target_id = form.value;
        if (form.value == 'None'){
            target_id = 7;
        }
        var voter_id = parseInt(localStorage.getItem('idx'));
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
        var voter_id = parseInt(localStorage.getItem('idx'));
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
        var voter_id = parseInt(localStorage.getItem('idx'));
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
        var id = localStorage.getItem("idx");
        fetch("/game/endspeak", {
            method: "POST",
            body: new URLSearchParams({id}),
        }).then((response) =>{
            if (response.ok) console.log("End Current Speaking Turn");
        });
        return;
    }) 
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

// for each to check which to mute and which to unmute 
function mute(){
    var w = document.getElementById('disabled').style.visibility = "visible";
    document.querySelector('.textmessage').disabled = true;
}
function unmute(){
    var w = document.getElementById('disabled').style.visibility = "hidden";
    document.querySelector('.textmessage').disabled = false;
}
/*Utilities*/

const heightOutput = document.querySelector("#height");
const widthOutput = document.querySelector("#width");
function reportWindowSize() {
    heightOutput.textContent = window.innerHeight;
    widthOutput.textContent = window.innerWidth;
}
window.onresize = reportWindowSize;


var slider = document.getElementById("myRange");
slider.onchange = function() {
    let i = 1;
    for(;i<=this.value;){
        document.querySelector(".player"+(i).toString()).style.visibility = "visible";
        i++;
    }
}
