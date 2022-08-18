import config from "./config.json";

function once(target: EventTarget, name: string) {
    return new Promise((resolve, reject) => {
        target.addEventListener(
            name,
            (e) => {
                resolve(e);
            },
            { once: true }
        );
    });
}

async function main() {
    console.log("Trying to connect to the backend..");

    const socket = new WebSocket(config.backendUrl);

    await once(socket, "open");
    console.log("Connected!");

    socket.send("ping");
    console.log("Sent ping");

    const response = await once(socket, "message");
    console.log("Received ", response);
}

window.onload = main;
