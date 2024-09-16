import  browser from "webextension-polyfill";
console.log("hello from the content script");   
browser.runtime.onMessage.addListener((message) => {
    console.log("Received message from background", message);
});