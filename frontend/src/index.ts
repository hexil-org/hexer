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
    if (process.env.NODE_ENV !== "production") {
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

    while (true) {
        const response = <MessageEvent>await once(socket, "message");
        const msg = JSON.parse(response.data);
        console.log("Received ", msg);
    }
}

window.onload = main;
