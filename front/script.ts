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
    const socket = new WebSocket("ws://localhost:8080");

    await once(socket, "open");

    socket.send("ping");
    console.log("Sent ping");

    const response = await once(socket, "message");
    console.log("Received ", response);
}

window.onload = main;
