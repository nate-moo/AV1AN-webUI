let prog = document.getElementById("progress");
let id = document.querySelector("span#identifier");


// setInterval(
//     () => fetch(`/api/data/${id.innerHTML}`).then(r => r.text()).then(a => prog.innerText = a),
//     1000
// )

const utf = new TextDecoder("utf-8");
let result;
let first = true;
fetch('/events')
    .then((response) => response.body)
    .then((stream) => {

        const reader = stream.getReader();
        reader.read().then(function processText({done, value}) {
            if (first) {
                first = false;
                return reader.read().then(processText)
            }
            const chunk = value;
            let decoded = utf.decode(chunk);
            let filtered = decoded.trim().slice(5)
            prog.textContent = prog.textContent + `\n${filtered}`;
            prog.scrollTop = prog.scrollHeight;

            result += chunk;

            // Read some more, and call this function again
            return reader.read().then(processText);
        });


    })

