import axios from "axios";

const CONCURRENT_USERS = 100;

const fetchSSE = async () => {
    try {
        const response = axios.get('http://127.0.0.1:8080/sse', {
            timeout: 1000,
            headers: {
                Accept: 'text/event-stream',
            }
        })
        console.log("Connected", (await response).status)
    } catch (error) {
        console.error("error streaming sse events", error);
    }
}

const runTest = async () => {
    const promise: Promise<void>[] = [];
    for (let i = 0; i < CONCURRENT_USERS; i++) {
        promise.push(fetchSSE())
    }
    await Promise.all(promise);
    console.log("all done!");
}

runTest();