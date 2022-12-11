
//post events
var endspeak = document.getElementById("endspeakturn");
var poison = true;
var antidote = true; 


/* => GAME LOOP*/ 

function mute(){
    var w = document.getElementById('disabled').style.visibility = "visible";
    document.querySelector('.textmessage').disabled = true;
}
function unmute(){
    var w = document.getElementById('disabled').style.visibility = "hidden";
    document.querySelector('.textmessage').disabled = false;
}


function rolesubmit(idparam){
    var form = document.getElementById(idparam);
    console.log(form.value);
}

function votesubmit(){
    var form = document.getElementById('voteoption');
    console.log(form.value);
}

function replace(turnparam) {
    empty = document.getElementById("empty");
    switch (turnparam) {
        case 1:
            empty.innerHTML = "<div class = 'vote' id = 'wolfkill'> Select Target <form> <select id='killoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick = \"rolesubmit('killoption')\" id = 'killconfirm'>Confirm</button> </form> Vote Out: <form> <select id='voteoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick = \"votesubmit()\" id = 'voteconfirm'>Confirm</button> </form> </div>";
            break;
        case 3:
            empty.innerHTML = "<div class = 'vote' id = 'reveal'> Select Target <form> <select id='revealoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick = \"rolesubmit('revealoption')\" id = 'revealconfirm'>Confirm</button> </form> Vote Out: <form> <select id='voteoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick = \"votesubmit()\" id = 'voteconfirm'>Confirm</button> </form> </div>";
            break;
        case 2:
            empty.innerHTML = "<div class = 'vote' id = 'witchchoice'> Choose: <div> <button class = 'choose' id = 'Antidote' type='submit'>Antidote</button> <button class = 'choose' id = 'Poison' type='submit'>Poison</button> </div> <form> <select id='witchoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick=\"rolesubmit('witchoption')\" id = 'witchconfirm'>Confirm</button> </form>Vote Out: <form> <select id='voteoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick = \"votesubmit()\" id = 'voteconfirm'>Confirm</button> </form> </div>";
            break;
        case 4:
            empty.innerHTML = "<div class = 'vote' id = 'civilianchoice'> Vote Out: <form> <select id='voteoption'> <option value='None' Selected>None</option> <option value='0'>Player 1</option> <option value='1'>Player 2</option> <option value='2'>Player 3</option> <option value='3'>Player 4</option> <option value='4'>Player 5</option> <option value='5'>Player 6</option> </select> <button type='submit' onclick = \"votesubmit()\" id = 'voteconfirm'>Confirm</button> </form> </div>";
        default:
            break;
    }
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
