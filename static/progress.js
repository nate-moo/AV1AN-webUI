let prog = document.getElementById("progress");
let id = document.querySelector("span#identifier");

setInterval(
    () => fetch(`/api/data/${id.innerHTML}`).then(r => r.text()).then(a => prog.innerText = a),
    1000
)

