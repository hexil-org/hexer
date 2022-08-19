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

function getBackendUrl() {
    if (location.host === "localhost:9451") {
        return "ws://localhost:9450";
    }

    const backendProtocol = location.protocol === "http" ? "ws" : "wss";
    return backendProtocol + "://" + location.host + location.pathname;
}

async function main() {
    console.log("Trying to connect to the backend..");

    const backendUrl = getBackendUrl();
    const socket = new WebSocket(backendUrl);

    await once(socket, "open");
    console.log("Connected!");

    socket.send("ping");
    console.log("Sent ping");

    const response = await once(socket, "message");
    console.log("Received ", response);
}

window.onload = main;
